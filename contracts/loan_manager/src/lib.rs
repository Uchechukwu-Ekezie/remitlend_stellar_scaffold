#![no_std]

use soroban_sdk::{
    contract, contractimpl, 
    contracttype, Address, Env, Vec, 
    token, contractevent,
};

mod nft {
    soroban_sdk::contractimport!(
        file = "../../target/stellar/local/remittance_nft.wasm"
    );
}

mod pool {
    soroban_sdk::contractimport!(
        file = "../../target/stellar/local/lending_pool.wasm"
    ); 
}

// use remittance_nft::{RemittanceNFTClient};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum LoanStatus {
    Pending = 0,
    Active = 1,
    Repaid = 2,
    Defaulted = 3,
}

#[contracttype]
#[derive(Clone)]
pub struct Loan {
    pub loan_id: u64,
    pub borrower: Address,
    pub nft_collateral_id: u64,
    pub loan_amount: i128,
    pub outstanding_balance: i128,
    pub total_repaid: i128,
    pub interest_rate: u32,          // APR in basis points
    pub duration_months: u32,
    pub monthly_payment: i128,
    pub start_timestamp: u64,
    pub next_payment_due: u64,
    pub status: LoanStatus,
    pub payments_made: u32,
    pub payments_missed: u32,
}

#[contracttype]
pub enum DataKey {
    LoanCounter,
    Loan(u64),
    BorrowerLoans(Address),
    RemittanceNFTContract,
    LendingPoolContract,
    OracleContract,
    USDCTokenAddress,
}


#[contractevent]
pub struct LoanRequestedEvent {
    pub borrower: Address,
    pub loan_id: u64,
}

#[contractevent]
pub struct LoanApprovedEvent {
    pub loan_id: u64,
}

#[contractevent]
pub struct PaymentMadeEvent {
    pub loan_id: u64,
    pub amount: i128,
}

#[contractevent]
pub struct PaymentMissedEvent {
    pub loan_id: u64,
    pub missed_count: u32,
}

#[contract]
pub struct LoanManager;

#[contractimpl]
impl LoanManager {

    pub fn __initialize(
        env: Env,
        nft_contract: Address,
        pool_contract: Address,
        oracle_contract: Address,
        usdc_token: Address,
    ) {
        env.storage().instance().set(&DataKey::RemittanceNFTContract, &nft_contract);
        env.storage().instance().set(&DataKey::LendingPoolContract, &pool_contract);
        env.storage().instance().set(&DataKey::OracleContract, &oracle_contract);
        env.storage().instance().set(&DataKey::USDCTokenAddress, &usdc_token);
        env.storage().instance().set(&DataKey::LoanCounter, &0u64);
    }

    // Request loan
    pub fn request_loan(
        env: Env,
        borrower: Address,
        nft_id: u64,
        amount: i128,
        duration_months: u32,
    ) -> u64 {
        borrower.require_auth();

        // Verify NFT ownership and calculate collateral value
        let _nft_contract: Address = env.storage().instance().get(&DataKey::RemittanceNFTContract).unwrap();
        let nft_client = nft::Client::new(&env, &_nft_contract);


        let nft_data = nft_client.get_nft_data(&nft_id);
        assert!(nft_data.owner == borrower, "NFT does not belong to borrower");

        // Call NFT contract to verify and get collateral value
        let nft_client = nft::Client::new(&env, &_nft_contract);
        let collateral_value = nft_client.get_nft_data(&nft_id);
        
        // if collateral_value
        if collateral_value.owner != borrower {
            
        }   

        // Calculate loan terms
        let interest_rate = Self::calculate_interest_rate(&env, nft_id);
        let monthly_payment = Self::calculate_monthly_payment(amount, interest_rate, duration_months);

        // Create loan
        let mut counter: u64 = env.storage().instance().get(&DataKey::LoanCounter).unwrap_or(0);
        counter += 1;

        let loan = Loan {
            loan_id: counter,
            borrower: borrower.clone(),
            nft_collateral_id: nft_id,
            loan_amount: amount,
            outstanding_balance: amount,
            total_repaid: 0,
            interest_rate,
            duration_months,
            monthly_payment,
            start_timestamp: env.ledger().timestamp(),
            next_payment_due: env.ledger().timestamp() + (30 * 24 * 60 * 60), // 30 days
            status: LoanStatus::Pending,
            payments_made: 0,
            payments_missed: 0,
        };

        env.storage().instance().set(&DataKey::LoanCounter, &counter);
        env.storage().instance().set(&DataKey::Loan(counter), &loan);

        // Track borrower loans
        let mut borrower_loans: Vec<u64> = env.storage().instance()
            .get(&DataKey::BorrowerLoans(borrower.clone()))
            .unwrap_or(Vec::new(&env));
        borrower_loans.push_back(counter);
        env.storage().instance().set(&DataKey::BorrowerLoans(borrower.clone()), &borrower_loans);

        env.events().publish(("loan_requested", borrower), counter);

        counter
    }

    // Approve and fund loan
    pub fn approve_loan(env: Env, loan_id: u64) {
        let mut loan: Loan = env.storage().instance()
            .get(&DataKey::Loan(loan_id))
            .expect("Loan does not exist");

        assert!(loan.status == LoanStatus::Pending, "Loan not pending");

        // Stake NFT
        let _nft_contract: Address = env.storage().instance().get(&DataKey::RemittanceNFTContract).unwrap();
        // Call _nft_contract.stake_nft(loan.nft_collateral_id, loan_id)

        // Borrow from pool
        let _pool_contract: Address = env.storage().instance().get(&DataKey::LendingPoolContract).unwrap();
        // Call _pool_contract.borrow(loan.loan_amount, loan.borrower, loan_id)

        // Update loan status
        loan.status = LoanStatus::Active;
        env.storage().instance().set(&DataKey::Loan(loan_id), &loan);

        env.events().publish(("loan_approved",), loan_id);
    }

    // Process payment
    pub fn make_payment(env: Env, loan_id: u64, amount: i128) {
        let mut loan: Loan = env.storage().instance()
            .get(&DataKey::Loan(loan_id))
            .expect("Loan does not exist");

        assert!(loan.status == LoanStatus::Active, "Loan not active");

        // Transfer USDC from borrower to pool
        let usdc_token: Address = env.storage().instance().get(&DataKey::USDCTokenAddress).unwrap();
        let pool_contract: Address = env.storage().instance().get(&DataKey::LendingPoolContract).unwrap();

        let usdc_client = token::Client::new(&env, &usdc_token);
        usdc_client.transfer(&loan.borrower, &pool_contract, &amount);

        // Calculate principal and interest split
        let interest_portion = Self::calculate_interest_portion(loan.outstanding_balance, loan.interest_rate);
        let principal_portion = if amount > interest_portion {
            amount - interest_portion
        } else {
            0
        };

        // Update loan
        loan.total_repaid += amount;
        loan.outstanding_balance -= principal_portion;
        loan.payments_made += 1;
        loan.next_payment_due += (30 * 24 * 60 * 60); // Next month

        // Check if fully repaid
        if loan.outstanding_balance <= 0 {
            loan.status = LoanStatus::Repaid;

            // Unstake NFT
            let _nft_contract: Address = env.storage().instance().get(&DataKey::RemittanceNFTContract).unwrap();
            // Call _nft_contract.unstake_nft(loan.nft_collateral_id)
            let nft_client = nft::Client::new(&env, &_nft_contract);
            nft_client.unstake_nft(&loan.nft_collateral_id);
        }

        // Notify pool of repayment
        let pool_contract: Address = env.storage().instance().get(&DataKey::LendingPoolContract).unwrap();
        let pool_client = pool::Client::new(&env, &pool_contract);
        pool_client.repay(&principal_portion, &interest_portion, &loan_id);
        // Use existing `pool_contract` variable above for notifications.
        // Call pool_contract.repay(principal_portion, interest_portion, loan_id)

        env.storage().instance().set(&DataKey::Loan(loan_id), &loan);

        env.events().publish(("payment_made", loan_id), amount);
    }

    // Process automatic repayment (called by Oracle)
    pub fn process_automatic_repayment(env: Env, loan_id: u64, remittance_amount: i128) -> i128 {
        let oracle: Address = env.storage().instance().get(&DataKey::OracleContract).unwrap();
        oracle.require_auth();

        let loan: Loan = env.storage().instance()
            .get(&DataKey::Loan(loan_id))
            .expect("Loan does not exist");

        let payment_amount = if remittance_amount >= loan.monthly_payment {
            loan.monthly_payment
        } else {
            remittance_amount
        };

        // Process payment
        Self::make_payment(env.clone(), loan_id, payment_amount);

        // Return remaining amount for recipient
        remittance_amount - payment_amount
    }

    // Mark payment as missed (called by Oracle)
    pub fn mark_payment_missed(env: Env, loan_id: u64) {
        let oracle: Address = env.storage().instance().get(&DataKey::OracleContract).unwrap();
        oracle.require_auth();

        let mut loan: Loan = env.storage().instance()
            .get(&DataKey::Loan(loan_id))
            .expect("Loan does not exist");

        loan.payments_missed += 1;

        // Check for default (2 consecutive missed payments)
        if loan.payments_missed >= 2 {
            loan.status = LoanStatus::Defaulted;
            // Liquidation logic would go here
        }

        env.storage().instance().set(&DataKey::Loan(loan_id), &loan);

        env.events().publish(("payment_missed", loan_id), loan.payments_missed);
    }

    // Get loan details
    pub fn get_loan(env: Env, loan_id: u64) -> Loan {
        env.storage().instance()
            .get(&DataKey::Loan(loan_id))
            .expect("Loan does not exist")
    }

    fn calculate_interest_rate(env: &Env, nft_id: u64) -> u32 {
        // Keep a reference to the NFT contract for potential future calls; underscore to silence unused variable warnings.
        let _nft_contract: Address = env.storage().instance().get(&DataKey::RemittanceNFTContract).unwrap();
        let nft_client = nft::Client::new(&env, &_nft_contract);
        
        // Call NFT contract to get NFT data
        let nft_data = nft_client.get_nft_data(&nft_id);

        // Placeholder scoring logic using nft_id 
        let score = (nft_data.reliability_score % 100) as u32;
        if score >= 90 {
            1500u32
        } else if score >= 80 {
            2000u32
        } else if score >= 70 {
            3000u32
        } else {
            4000u32
        }
    }

    // Internal: Calculate monthly payment
    fn calculate_monthly_payment(principal: i128, annual_rate_bps: u32, months: u32) -> i128 {
        // Monthly rate = annual rate / 12
        let monthly_rate_bps = annual_rate_bps / 12;

        // Simple interest calculation for MVP
        let total_interest = (principal * (annual_rate_bps as i128) * (months as i128)) / (12 * 10000);
        let total_repayment = principal + total_interest;

        total_repayment / (months as i128)
    }

    // Internal: Calculate interest portion of payment
    fn calculate_interest_portion(outstanding: i128, annual_rate_bps: u32) -> i128 {
        let monthly_rate_bps = annual_rate_bps / 12;
        (outstanding * (monthly_rate_bps as i128)) / 10000
    }
}
