
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, Symbol,
};

const LOANS: Symbol = symbol_short!("LOANS");
const ADMIN: Symbol = symbol_short!("ADMIN");

/// Represents a single micro-loan to a farmer
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loan {
    pub farmer: Address,
    pub principal: i128,       // Original amount disbursed
    pub repaid: i128,          // Amount repaid so far
    pub fully_repaid: bool,
}

#[contract]
pub struct SeedPoolContract;

#[contractimpl]
impl SeedPoolContract {
    /// NGO or cooperative admin initializes the lending contract
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
        let loans: Map<Address, Loan> = Map::new(&env);
        env.storage().instance().set(&LOANS, &loans);
    }

    /// Admin issues a micro-loan to a farmer for seed or fertilizer purchase.
    /// USDC is transferred directly to farmer's wallet. Loan record is stored on-chain.
    pub fn issue_loan(
        env: Env,
        admin: Address,
        farmer: Address,
        token: Address,
        amount: i128,
    ) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if admin != stored_admin {
            panic!("unauthorized");
        }
        if amount <= 0 {
            panic!("amount must be positive");
        }

        let mut loans: Map<Address, Loan> =
            env.storage().instance().get(&LOANS).unwrap();

        // Prevent duplicate active loan for same farmer
        if loans.contains_key(farmer.clone()) {
            let existing: Loan = loans.get(farmer.clone()).unwrap();
            if !existing.fully_repaid {
                panic!("farmer has active loan");
            }
        }

        // Disburse USDC from admin/contract to farmer
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &farmer, &amount);

        // Record loan on-chain
        let loan = Loan {
            farmer: farmer.clone(),
            principal: amount,
            repaid: 0,
            fully_repaid: false,
        };
        loans.set(farmer, loan);
        env.storage().instance().set(&LOANS, &loans);
    }

    /// Farmer repays part or all of the loan.
    /// Partial repayments are tracked; full repayment marks loan as closed.
    pub fn repay(env: Env, farmer: Address, token: Address, amount: i128) {
        farmer.require_auth();

        if amount <= 0 {
            panic!("repay amount must be positive");
        }

        let mut loans: Map<Address, Loan> =
            env.storage().instance().get(&LOANS).unwrap();

        let mut loan: Loan = loans.get(farmer.clone()).expect("no loan found");

        if loan.fully_repaid {
            panic!("loan already repaid");
        }

        let remaining = loan.principal - loan.repaid;
        let actual_repay = if amount > remaining { remaining } else { amount };

        // Transfer USDC from farmer back to contract (repayment pool)
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&farmer, &env.current_contract_address(), &actual_repay);

        loan.repaid += actual_repay;
        if loan.repaid >= loan.principal {
            loan.fully_repaid = true;
        }

        loans.set(farmer, loan);
        env.storage().instance().set(&LOANS, &loans);
    }

    /// Check a farmer's loan status
    pub fn get_loan(env: Env, farmer: Address) -> Loan {
        let loans: Map<Address, Loan> =
            env.storage().instance().get(&LOANS).unwrap();
        loans.get(farmer).expect("no loan found")
    }
}
