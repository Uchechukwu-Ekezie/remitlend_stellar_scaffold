#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

#[contracttype]
#[derive(Clone)]
pub struct LenderInfo {
    pub deposit_amount: i128,
    pub deposit_timestamp: u64,
    pub earned_interest: i128,
    pub share_percentage: u32,  // in basis points (10000 = 100%)
}

#[contracttype]
pub enum DataKey {
    TotalLiquidity,
    TotalBorrowed,
    TotalInterestEarned,
    LenderInfo(Address),
    LoanManagerAddress,
    USDCTokenAddress,
    BaseInterestRate,
    MaxUtilization,
}

#[contract]
pub struct LendingPool;

#[contractimpl]
impl LendingPool {

    pub fn __initialize(
        env: Env,
        loan_manager: Address,
        usdc_token: Address,
        base_rate: u32,  // in basis points, e.g. 800 = 8%
    ) {
        env.storage().instance().set(&DataKey::LoanManagerAddress, &loan_manager);
        env.storage().instance().set(&DataKey::USDCTokenAddress, &usdc_token);
        env.storage().instance().set(&DataKey::BaseInterestRate, &base_rate);
        env.storage().instance().set(&DataKey::MaxUtilization, &9000u32); // 90%
        env.storage().instance().set(&DataKey::TotalLiquidity, &0i128);
        env.storage().instance().set(&DataKey::TotalBorrowed, &0i128);
        env.storage().instance().set(&DataKey::TotalInterestEarned, &0i128);
    }

    pub fn deposit(env: Env, lender: Address, amount: i128) {
        lender.require_auth();
        
        assert!(amount > 0, "Amount must be positive");
        
        // Transfer USDC from lender to contract
        let usdc_token: Address = env.storage().instance().get(&DataKey::USDCTokenAddress).unwrap();
        let contract_address = env.current_contract_address();
        
        let usdc_client = token::Client::new(&env, &usdc_token);
        usdc_client.transfer(&lender, &contract_address, &amount);
        
        // Update lender info
        let mut lender_info: LenderInfo = env.storage().instance()
            .get(&DataKey::LenderInfo(lender.clone()))
            .unwrap_or(LenderInfo {
                deposit_amount: 0,
                deposit_timestamp: env.ledger().timestamp(),
                earned_interest: 0,
                share_percentage: 0,
            });
        
        lender_info.deposit_amount += amount;
        
        // Update total liquidity
        let mut total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap();
        total_liquidity += amount;
        
        // Update share percentage
        lender_info.share_percentage = ((lender_info.deposit_amount * 10000) / total_liquidity) as u32;
        
        env.storage().instance().set(&DataKey::LenderInfo(lender.clone()), &lender_info);
        env.storage().instance().set(&DataKey::TotalLiquidity, &total_liquidity);
        
        env.events().publish(("deposit", lender), amount);
    }
    
    // Lender withdraws USDC
    pub fn withdraw(env: Env, lender: Address, amount: i128) {
        lender.require_auth();
        
        let lender_info: LenderInfo = env.storage().instance()
            .get(&DataKey::LenderInfo(lender.clone()))
            .expect("No deposits found");
        
        assert!(lender_info.deposit_amount >= amount, "Insufficient balance");
        
        // Check available liquidity
        let available = Self::get_available_liquidity(env.clone());
        assert!(available >= amount, "Insufficient pool liquidity");
        
        // Transfer USDC to lender
        let usdc_token: Address = env.storage().instance().get(&DataKey::USDCTokenAddress).unwrap();
        let contract_address = env.current_contract_address();
        
        let usdc_client = token::Client::new(&env, &usdc_token);
        usdc_client.transfer(&contract_address, &lender, &amount);
        
        // Update lender info
        let mut updated_info = lender_info;
        updated_info.deposit_amount -= amount;
        env.storage().instance().set(&DataKey::LenderInfo(lender.clone()), &updated_info);
        
        // Update total liquidity
        let mut total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap();
        total_liquidity -= amount;
        env.storage().instance().set(&DataKey::TotalLiquidity, &total_liquidity);
        
        env.events().publish(("withdraw", lender), amount);
    }
    
    // Borrow from pool (called by LoanManager only)
    pub fn borrow(env: Env, amount: i128, borrower: Address, loan_id: u64) {
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerAddress).unwrap();
        loan_manager.require_auth();
        
        // Check available liquidity
        let available = Self::get_available_liquidity(env.clone());
        assert!(available >= amount, "Insufficient liquidity");
        
        // Check utilization won't exceed max
        let total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap();
        let mut total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap();
        let new_borrowed = total_borrowed + amount;
        let new_utilization = ((new_borrowed * 10000) / total_liquidity) as u32;
        
        let max_utilization: u32 = env.storage().instance().get(&DataKey::MaxUtilization).unwrap();
        assert!(new_utilization <= max_utilization, "Max utilization exceeded");
        
        // Transfer USDC to borrower
        let usdc_token: Address = env.storage().instance().get(&DataKey::USDCTokenAddress).unwrap();
        let contract_address = env.current_contract_address();
        
        let usdc_client = token::Client::new(&env, &usdc_token);
        usdc_client.transfer(&contract_address, &borrower, &amount);
        
        // Update total borrowed
        total_borrowed += amount;
        env.storage().instance().set(&DataKey::TotalBorrowed, &total_borrowed);
        
        env.events().publish(("borrow", loan_id, borrower), amount);
    }
    
    // Repay to pool (called by LoanManager only)
    pub fn repay(env: Env, principal: i128, interest: i128, loan_id: u64) {
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerAddress).unwrap();
        loan_manager.require_auth();
        
        // Update total borrowed
        let mut total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap();
        total_borrowed -= principal;
        env.storage().instance().set(&DataKey::TotalBorrowed, &total_borrowed);
        
        // Update total interest earned
        let mut total_interest: i128 = env.storage().instance().get(&DataKey::TotalInterestEarned).unwrap();
        total_interest += interest;
        env.storage().instance().set(&DataKey::TotalInterestEarned, &total_interest);
        
        env.events().publish(("repay", loan_id), principal + interest);
    }
    
    // Get available liquidity
    pub fn get_available_liquidity(env: Env) -> i128 {
        let total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap_or(0);
        let total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap_or(0);
        total_liquidity - total_borrowed
    }
    
    // Get lender info
    pub fn get_lender_info(env: Env, lender: Address) -> LenderInfo {
        env.storage().instance()
            .get(&DataKey::LenderInfo(lender))
            .unwrap_or(LenderInfo {
                deposit_amount: 0,
                deposit_timestamp: 0,
                earned_interest: 0,
                share_percentage: 0,
            })
    }
    
    // Get utilization rate
    pub fn get_utilization_rate(env: Env) -> u32 {
        let total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap_or(1);
        let total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap_or(0);
        
        if total_liquidity == 0 {
            return 0;
        }
        
        ((total_borrowed * 10000) / total_liquidity) as u32
    }
}