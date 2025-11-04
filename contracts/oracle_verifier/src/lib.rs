
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, String, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct VerificationRequest {
    pub user: Address,
    pub provider: String,           // "wise", "western_union", "paypal"
    pub account_id: String,         // User's account with provider
    pub request_timestamp: u64,
    pub status: VerificationStatus,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum VerificationStatus {
    Pending = 0,
    Verified = 1,
    Failed = 2,
}

#[contracttype]
#[derive(Clone)]
pub struct PaymentRecord {
    pub month_index: u32,
    pub paid: bool,
}

#[contracttype]
pub enum DataKey {
    VerificationRequest(Address),
    OracleOperators(u32),           // Index -> Address of authorized operators
    OracleOperatorCount,
    RemittanceNFTContract,
    LoanManagerContract,
    MonitoredLoans(u64),            // loan_id -> bool (is being monitored)
}

#[contract]
pub struct OracleVerifier;

#[contractimpl]
impl OracleVerifier {

    pub fn __initialize(
        env: Env,
        nft_contract: Address,
        loan_manager: Address,
        operators: Vec<Address>,
    ) {
        env.storage().instance().set(&DataKey::RemittanceNFTContract, &nft_contract);
        env.storage().instance().set(&DataKey::LoanManagerContract, &loan_manager);
        
        // Set oracle operators
        let count = operators.len();
        for i in 0..count {
            env.storage().instance().set(&DataKey::OracleOperators(i), &operators.get(i).unwrap());
        }
        env.storage().instance().set(&DataKey::OracleOperatorCount, &count);
    }

    pub fn request_verification(
        env: Env,
        user: Address,
        provider: String,
        account_id: String,
    ) {
        user.require_auth();
        
        let request = VerificationRequest {
            user: user.clone(),
            provider,
            account_id,
            request_timestamp: env.ledger().timestamp(),
            status: VerificationStatus::Pending,
        };
        
        env.storage().instance().set(&DataKey::VerificationRequest(user.clone()), &request);
        
        env.events().publish(("verification_requested",), user);
    }
    
    // Oracle operator submits verification result
    pub fn submit_verification(
        env: Env,
        operator: Address,
        user: Address,
        monthly_amount: i128,
        history_months: u32,
        total_sent: i128,
        payment_history: Vec<PaymentRecord>,
    ) {
        // Verify operator is authorized
        Self::verify_operator(&env, &operator);
        operator.require_auth();
        
        let mut request: VerificationRequest = env.storage().instance()
            .get(&DataKey::VerificationRequest(user.clone()))
            .expect("No verification request found");
        
        assert!(request.status == VerificationStatus::Pending, "Already processed");
        
        // Calculate reliability score
        let reliability_score = Self::calculate_reliability_score(&payment_history);
        
        // Call RemittanceNFT contract to mint
        let nft_contract: Address = env.storage().instance().get(&DataKey::RemittanceNFTContract).unwrap();
        
        // In real implementation:
        // let nft_client = RemittanceNFTClient::new(&env, &nft_contract);
        // let token_id = nft_client.mint(
        //     &user,
        //     &monthly_amount,
        //     &reliability_score,
        //     &history_months,
        //     &total_sent,
        //     &payment_history,
        // );
        
        // Update request status
        request.status = VerificationStatus::Verified;
        env.storage().instance().set(&DataKey::VerificationRequest(user.clone()), &request);
        
        env.events().publish(("verification_complete", user), reliability_score);
    }
    
    // Start monitoring loan for automatic repayments
    pub fn start_monitoring_loan(env: Env, loan_id: u64) {
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerContract).unwrap();
        loan_manager.require_auth();
        
        env.storage().instance().set(&DataKey::MonitoredLoans(loan_id), &true);
        
        env.events().publish(("monitoring_started",), loan_id);
    }
    
    // Oracle detects remittance and triggers automatic repayment
    pub fn report_remittance(
        env: Env,
        operator: Address,
        user: Address,
        nft_id: u64,
        amount: i128,
        loan_id: u64,
    ) {
        Self::verify_operator(&env, &operator);
        operator.require_auth();
        
        // Check if loan is being monitored
        let is_monitored: bool = env.storage().instance()
            .get(&DataKey::MonitoredLoans(loan_id))
            .unwrap_or(false);
        
        assert!(is_monitored, "Loan not being monitored");
        
        // Update NFT with new remittance
        let nft_contract: Address = env.storage().instance().get(&DataKey::RemittanceNFTContract).unwrap();
        // nft_contract.update_remittance_data(nft_id, amount, updated_total)
        
        // Process automatic repayment through LoanManager
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerContract).unwrap();
        // let remaining = loan_manager.process_automatic_repayment(loan_id, amount)
        
        env.events().publish(("remittance_reported", loan_id, nft_id), amount);
    }
    
    // Oracle reports missed payment
    pub fn report_missed_payment(
        env: Env,
        operator: Address,
        loan_id: u64,
        nft_id: u64,
    ) {
        Self::verify_operator(&env, &operator);
        operator.require_auth();
        
        // Update NFT
        let nft_contract: Address = env.storage().instance().get(&DataKey::RemittanceNFTContract).unwrap();
        // nft_contract.mark_payment_missed(nft_id)
        
        // Update loan
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerContract).unwrap();
        // loan_manager.mark_payment_missed(loan_id)
        
        env.events().publish(("payment_missed_reported", loan_id), nft_id);
    }
    
    // Get verification status
    pub fn get_verification_status(env: Env, user: Address) -> VerificationStatus {
        let request: VerificationRequest = env.storage().instance()
            .get(&DataKey::VerificationRequest(user))
            .expect("No verification request found");
        
        request.status
    }
    
    // Internal: Verify operator is authorized
    fn verify_operator(env: &Env, operator: &Address) {
        let count: u32 = env.storage().instance().get(&DataKey::OracleOperatorCount).unwrap();
        
        for i in 0..count {
            let authorized: Address = env.storage().instance().get(&DataKey::OracleOperators(i)).unwrap();
            if &authorized == operator {
                return;
            }
        }
        
        panic!("Unauthorized operator");
    }
    
    // Internal: Calculate reliability score from payment history
    fn calculate_reliability_score(payment_history: &Vec<PaymentRecord>) -> u32 {
        let mut paid_count = 0u32;
        let total_count = payment_history.len() as u32;
        
        if total_count == 0 {
            return 100;
        }
        
        for i in 0..payment_history.len() {
            if payment_history.get(i).unwrap().paid {
                paid_count += 1;
            }
        }
        
        (paid_count * 100) / total_count
    }
}


