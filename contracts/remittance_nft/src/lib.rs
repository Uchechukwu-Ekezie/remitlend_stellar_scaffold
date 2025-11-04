#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct RemittanceData {
    pub owner: Address,
    pub monthly_amount: i128,          // Amount in stroops (1 XLM = 10,000,000 stroops)
    pub reliability_score: u32,        // 0-100
    pub history_months: u32,
    pub total_sent: i128,
    pub last_remittance_timestamp: u64,
    pub lifetime_missed_payments: u32,
    pub is_staked: bool,
    pub staked_in_loan: u64,          // loan_id if staked
}

#[contracttype]
#[derive(Clone)]
pub struct PaymentRecord {
    pub month_index: u32,
    pub paid: bool,                    // true = paid, false = missed
}

#[contracttype]
pub enum DataKey {
    TokenCounter,
    RemittanceData(u64),              // token_id -> RemittanceData
    PaymentHistory(u64),              // token_id -> Vec<PaymentRecord>
    OracleAddress,
    LoanManagerAddress,
}


#[contract]
pub struct RemittanceNFT;

#[contractimpl]
impl RemittanceNFT {

    pub fn __initialize(env: Env, oracle: Address, loan_manager: Address) {
        env.storage().instance().set(&DataKey::OracleAddress, &oracle);
        env.storage().instance().set(&DataKey::LoanManagerAddress, &loan_manager);
        env.storage().instance().set(&DataKey::TokenCounter, &0u64);
    }

     pub fn mint(
        env: Env,
        owner: Address,
        monthly_amount: i128,
        reliability_score: u32,
        history_months: u32,
        total_sent: i128,
        payment_history: Vec<PaymentRecord>,
    ) -> u64 {
        owner.require_auth();
        
        // Get and increment token counter
        let mut counter: u64 = env.storage().instance().get(&DataKey::TokenCounter).unwrap_or(0);
        counter += 1;
        
        // Create remittance data
        let data = RemittanceData {
            owner: owner.clone(),
            monthly_amount,
            reliability_score,
            history_months,
            total_sent,
            last_remittance_timestamp: env.ledger().timestamp(),
            lifetime_missed_payments: Self::count_missed_payments(&payment_history),
            is_staked: false,
            staked_in_loan: 0,
        };
        
        // Store data
        env.storage().instance().set(&DataKey::TokenCounter, &counter);
        env.storage().instance().set(&DataKey::RemittanceData(counter), &data);
        env.storage().instance().set(&DataKey::PaymentHistory(counter), &payment_history);
        
        // Emit event
        env.events().publish(("mint_nft", owner), counter);
        
        counter
    }
    
    // Stake NFT as loan collateral (called by LoanManager only)
    pub fn stake_nft(env: Env, token_id: u64, loan_id: u64) {
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerAddress).unwrap();
        loan_manager.require_auth();
        
        let mut data: RemittanceData = env.storage().instance()
            .get(&DataKey::RemittanceData(token_id))
            .expect("NFT does not exist");
        
        assert!(!data.is_staked, "NFT already staked");
        
        data.is_staked = true;
        data.staked_in_loan = loan_id;
        
        env.storage().instance().set(&DataKey::RemittanceData(token_id), &data);
        env.events().publish(("stake_nft", token_id), loan_id);
    }
    
    // Unstake NFT after loan repayment (called by LoanManager only)
    pub fn unstake_nft(env: Env, token_id: u64) {
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerAddress).unwrap();
        loan_manager.require_auth();
        
        let mut data: RemittanceData = env.storage().instance()
            .get(&DataKey::RemittanceData(token_id))
            .expect("NFT does not exist");
        
        assert!(data.is_staked, "NFT not staked");
        
        data.is_staked = false;
        data.staked_in_loan = 0;
        
        env.storage().instance().set(&DataKey::RemittanceData(token_id), &data);
        env.events().publish(("unstake_nft",), token_id);
    }
    
    // Update remittance data (called by Oracle only)
    pub fn update_remittance_data(
        env: Env,
        token_id: u64,
        new_monthly_amount: i128,
        new_total_sent: i128,
    ) {
        let oracle: Address = env.storage().instance().get(&DataKey::OracleAddress).unwrap();
        oracle.require_auth();
        
        let mut data: RemittanceData = env.storage().instance()
            .get(&DataKey::RemittanceData(token_id))
            .expect("NFT does not exist");
        
        let mut payment_history: Vec<PaymentRecord> = env.storage().instance()
            .get(&DataKey::PaymentHistory(token_id))
            .unwrap();
        
        // Add new payment to history
        payment_history.push_back(PaymentRecord {
            month_index: data.history_months + 1,
            paid: true,
        });
        
        // Keep only last 24 months
        if payment_history.len() > 24 {
            payment_history.remove(0);
        }
        
        // Update data
        data.monthly_amount = new_monthly_amount;
        data.total_sent = new_total_sent;
        data.history_months += 1;
        data.last_remittance_timestamp = env.ledger().timestamp();
        data.reliability_score = Self::calculate_score(&env, &payment_history, data.lifetime_missed_payments);
        
        env.storage().instance().set(&DataKey::RemittanceData(token_id), &data);
        env.storage().instance().set(&DataKey::PaymentHistory(token_id), &payment_history);
        
        env.events().publish(("update_nft", token_id), data.reliability_score);
    }
    
    // Mark payment as missed (called by Oracle only)
    pub fn mark_payment_missed(env: Env, token_id: u64) {
        let oracle: Address = env.storage().instance().get(&DataKey::OracleAddress).unwrap();
        oracle.require_auth();
        
        let mut data: RemittanceData = env.storage().instance()
            .get(&DataKey::RemittanceData(token_id))
            .expect("NFT does not exist");
        
        let mut payment_history: Vec<PaymentRecord> = env.storage().instance()
            .get(&DataKey::PaymentHistory(token_id))
            .unwrap();
        
        // Add missed payment
        payment_history.push_back(PaymentRecord {
            month_index: data.history_months + 1,
            paid: false,
        });
        
        // Keep only last 24 months
        if payment_history.len() > 24 {
            payment_history.remove(0);
        }
        
        data.history_months += 1;
        data.lifetime_missed_payments += 1;
        data.reliability_score = Self::calculate_score(&env, &payment_history, data.lifetime_missed_payments);
        
        env.storage().instance().set(&DataKey::RemittanceData(token_id), &data);
        env.storage().instance().set(&DataKey::PaymentHistory(token_id), &payment_history);
        
        env.events().publish(("payment_missed", token_id), data.reliability_score);
    }
    
    // Get NFT data (public view)
    pub fn get_nft_data(env: Env, token_id: u64) -> RemittanceData {
        env.storage().instance()
            .get(&DataKey::RemittanceData(token_id))
            .expect("NFT does not exist")
    }
    
    // Calculate collateral value
    pub fn calculate_collateral_value(env: Env, token_id: u64, duration_months: u32) -> i128 {
        let data: RemittanceData = Self::get_nft_data(env, token_id);
        
        // Formula: monthly_amount × duration × (score/100) × 0.70
        let base_value = data.monthly_amount * (duration_months as i128);
        let score_adjusted = (base_value * (data.reliability_score as i128)) / 100;
        let final_value = (score_adjusted * 70) / 100;
        
        final_value
    }
    
    // Internal: Calculate reliability score
    fn calculate_score(env: &Env, payment_history: &Vec<PaymentRecord>, lifetime_missed: u32) -> u32 {
        // Count payments in last 24 months
        let mut paid_count = 0u32;
        let total_count = payment_history.len() as u32;
        
        for i in 0..payment_history.len() {
            if payment_history.get(i).unwrap().paid {
                paid_count += 1;
            }
        }
        
        // Recent score (last 24 months)
        let recent_score = if total_count > 0 {
            (paid_count * 100) / total_count
        } else {
            100
        };
        
        // Apply lifetime penalty
        let penalty = Self::calculate_lifetime_penalty(lifetime_missed);
        
        if recent_score > penalty {
            recent_score - penalty
        } else {
            0
        }
    }
    
    // Internal: Calculate lifetime penalty
    fn calculate_lifetime_penalty(lifetime_missed: u32) -> u32 {
        match lifetime_missed {
            0 => 0,
            1 => 2,
            2 => 5,  // -2 for first, -3 for second
            3 => 9,  // -2, -3, -4
            _ => 9 + ((lifetime_missed - 3) * 5),  // -5 for each additional
        }
    }
    
    // Internal: Count missed payments
    fn count_missed_payments(payment_history: &Vec<PaymentRecord>) -> u32 {
        let mut count = 0u32;
        for i in 0..payment_history.len() {
            if !payment_history.get(i).unwrap().paid {
                count += 1;
            }
        }
        count
    }
}