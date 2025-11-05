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
    AccumulatedInterestPerShare,
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
        let usdc_token_address: Address = env.storage().instance().get(&DataKey::USDCTokenAddress).unwrap();
        let usdc_token = token::Client::new(&env, &usdc_token_address);
        
        // Transfer tokens from lender to contract
        usdc_token.transfer(&lender, &env.current_contract_address(), &amount);
        
        // Update lender info
        let mut lender_info = env.storage().persistent()
            .get(&DataKey::LenderInfo(lender.clone()))
            .unwrap_or_else(|| LenderInfo {
                deposit_amount: 0,
                deposit_timestamp: env.ledger().timestamp(),
                earned_interest: 0,
                share_percentage: 0,
            });
            
        // Update total liquidity
        let mut total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap_or(0);
        total_liquidity += amount;
        env.storage().instance().set(&DataKey::TotalLiquidity, &total_liquidity);
        
        // Update lender's deposit amount
        lender_info.deposit_amount += amount;
        
        // Update share percentage
        if total_liquidity > 0 {
            lender_info.share_percentage = ((lender_info.deposit_amount as u128 * 10000) / total_liquidity as u128) as u32;
        } else {
            lender_info.share_percentage = 10000; // 100% if first depositor
        }
        
        env.storage().persistent().set(&DataKey::LenderInfo(lender.clone()), &lender_info);
        
        // Emit deposit event
        env.events().publish(("deposit", lender.clone()), amount);
        
        lender_info.deposit_amount += amount;
        
        // Update total liquidity
        let mut total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap();
        total_liquidity += amount;
        
        // Update share percentage
        lender_info.share_percentage = ((lender_info.deposit_amount * 10000) / total_liquidity) as u32;
        
        env.storage().instance().set(&DataKey::LenderInfo(lender.clone()), &lender_info);
        env.storage().instance().set(&DataKey::TotalLiquidity, &total_liquidity);
        
        env.events().publish(("deposit", lender.clone())    , amount);
    }
    
    // Lender withdraws USDC
    pub fn withdraw(env: Env, lender: Address, amount: i128) {
        lender.require_auth();
        
        assert!(amount > 0, "Amount must be positive");
        
        let mut lender_info: LenderInfo = env.storage().persistent()
            .get(&DataKey::LenderInfo(lender.clone()))
            .expect("Lender not found");
            
        assert!(lender_info.deposit_amount >= amount, "Insufficient balance");
        
        // Get available liquidity
        let total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap_or(0);
        let total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap_or(0);
        let available_liquidity = total_liquidity - total_borrowed;
        
        assert!(amount <= available_liquidity, "Insufficient liquidity in the pool");
        
        // Transfer USDC back to lender
        let usdc_token_address: Address = env.storage().instance().get(&DataKey::USDCTokenAddress).unwrap();
        let usdc_token = token::Client::new(&env, &usdc_token_address);
        
        // Calculate and transfer earned interest first
        let current_interest_per_share: i128 = env.storage().instance()
            .get(&DataKey::AccumulatedInterestPerShare)
            .unwrap_or(0);
            
        // Calculate pending interest based on the lender's current deposit
        let pending_interest = (lender_info.deposit_amount as i128 * current_interest_per_share) / 1_000_000_000
            - lender_info.earned_interest;
        
        // Transfer principal and interest
        if pending_interest > 0 {
            // Transfer principal + interest
            usdc_token.transfer(
                &env.current_contract_address(), 
                &lender, 
                &(amount + pending_interest)
            );
            
            // Update total interest earned
            let total_interest_earned: i128 = env.storage().instance()
                .get(&DataKey::TotalInterestEarned)
                .unwrap_or(0);
            
            if total_interest_earned >= pending_interest {
                env.storage().instance().set(
                    &DataKey::TotalInterestEarned,
                    &(total_interest_earned - pending_interest)
                );
            }
            
            lender_info.earned_interest += pending_interest;
        } else {
            // Only transfer principal if no interest is due
            usdc_token.transfer(&env.current_contract_address(), &lender, &amount);
        }
        
        // Update lender info
        lender_info.deposit_amount -= amount;
        
        // Update total liquidity
        let new_total_liquidity = total_liquidity - amount;
        env.storage().instance().set(&DataKey::TotalLiquidity, &new_total_liquidity);
        
        // Update share percentage if there's still liquidity
        if new_total_liquidity > 0 {
            lender_info.share_percentage = ((lender_info.deposit_amount as u128 * 10000) / new_total_liquidity as u128) as u32;
        } else {
            lender_info.share_percentage = 0;
        }
        
        // Save updated lender info
        env.storage().persistent().set(&DataKey::LenderInfo(lender.clone()), &lender_info);
        
        // Emit withdraw event
        env.events().publish(("withdraw", lender.clone()), amount);
    }
    
    // Borrow from pool (called by LoanManager only)
    pub fn borrow(env: Env, amount: i128, borrower: Address, loan_id: u64) {
        // Only loan manager can call this
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerAddress).unwrap();
        loan_manager.require_auth();
        
        assert!(amount > 0, "Amount must be positive");
        
        // Check available liquidity
        let total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap_or(0);
        let total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap_or(0);
        let available = total_liquidity - total_borrowed;
        
        assert!(amount <= available, "Insufficient liquidity in the pool");
        
        // Update total borrowed
        env.storage().instance().set(&DataKey::TotalBorrowed, &(total_borrowed + amount));
        
        // Transfer USDC to borrower
        let usdc_token_address: Address = env.storage().instance().get(&DataKey::USDCTokenAddress).unwrap();
        let usdc_token = token::Client::new(&env, &usdc_token_address);
        usdc_token.transfer(&env.current_contract_address(), &borrower, &amount);
        
        env.events().publish(("borrow", loan_id, borrower), amount);
    }
    
    // Repay to pool (called by LoanManager only)
    pub fn repay(env: Env, principal: i128, interest: i128, loan_id: u64) {
        // Only loan manager can call this
        let loan_manager: Address = env.storage().instance().get(&DataKey::LoanManagerAddress).unwrap();
        loan_manager.require_auth();
        
        assert!(principal >= 0 && interest >= 0, "Amounts must be non-negative");
        
        let total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap_or(0);
        let total_interest_earned: i128 = env.storage().instance().get(&DataKey::TotalInterestEarned).unwrap_or(0);
        
        // Update totals
        env.storage().instance().set(&DataKey::TotalBorrowed, &(total_borrowed - principal));
        env.storage().instance().set(&DataKey::TotalInterestEarned, &(total_interest_earned + interest));
        
        // Distribute interest to lenders proportionally
        if interest > 0 {
            let total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap_or(1);
            
            if total_liquidity > 0 {
                // In a real implementation, you would typically have a way to iterate through all lenders
                // For this example, we'll store the total interest and distribute it when lenders withdraw
                // or provide a separate claim function
                
                // Store the interest to be distributed
                let interest_per_share = (interest as u128).checked_mul(1_000_000_000)  // For better precision
                    .map(|v| v / (total_liquidity as u128))
                    .unwrap_or(0) as i128;
                
                // Store the current interest per share
                let mut current_interest_per_share: i128 = env.storage().instance()
                    .get(&DataKey::AccumulatedInterestPerShare)
                    .unwrap_or(0);
                    
                current_interest_per_share += interest_per_share;
                env.storage().instance().set(
                    &DataKey::AccumulatedInterestPerShare,
                    &current_interest_per_share
                );
                
                // The actual interest will be calculated when lenders withdraw or claim
                // by comparing their last claimed interest per share with the current one
            }
        }
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
        env.storage().persistent()
            .get(&DataKey::LenderInfo(lender))
            .unwrap_or_else(|| LenderInfo {
                deposit_amount: 0,
                deposit_timestamp: 0,
                earned_interest: 0,
                share_percentage: 0,
            })
    }
    
    // Get utilization rate
    pub fn get_utilization_rate(env: Env) -> u32 {
        let total_liquidity: i128 = env.storage().instance().get(&DataKey::TotalLiquidity).unwrap_or(0);
        let total_borrowed: i128 = env.storage().instance().get(&DataKey::TotalBorrowed).unwrap_or(0);
        
        if total_liquidity == 0 {
            return 0;
        }
        
        ((total_borrowed as u128 * 10000) / total_liquidity as u128) as u32
    }
}