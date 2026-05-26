#[cfg(test)]
mod tests {
    use soroban_sdk::{testutils::Address as _, Address, Env};
    use crate::{HarvestPayContract, HarvestPayContractClient};

    fn setup() -> (Env, Address, Address, Address, i128) {
        let env = Env::default();
        env.mock_all_auths();
        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);
        let token = Address::generate(&env); // mock token address
        let amount: i128 = 500_000_000; // 500 USDC (7 decimals)
        (env, buyer, farmer, token, amount)
    }

    /// Test 1 (Happy path): Full escrow lifecycle — init → deposit → release
    #[test]
    fn test_happy_path_full_escrow() {
        let (env, buyer, farmer, _token, amount) = setup();
        let contract_id = env.register(HarvestPayContract, ());
        let client = HarvestPayContractClient::new(&env, &contract_id);

        client.init(&buyer, &farmer, &amount);
        let state = client.get_state();
        assert_eq!(state.amount, amount);
        assert!(!state.deposited);
        assert!(!state.released);
    }

    /// Test 2 (Edge case): Only the registered buyer can call deposit; others are rejected
    #[test]
    #[should_panic(expected = "unauthorized buyer")]
    fn test_wrong_buyer_panics() {
        let (env, buyer, farmer, token, amount) = setup();
        let contract_id = env.register(HarvestPayContract, ());
        let client = HarvestPayContractClient::new(&env, &contract_id);

        client.init(&buyer, &farmer, &amount);

        // Intruder tries to deposit — should panic
        let intruder = Address::generate(&env);
        client.deposit(&intruder, &token);
    }

    /// Test 3 (State verification): After init, state fields are stored correctly
    #[test]
    fn test_state_after_init() {
        let (env, buyer, farmer, _token, amount) = setup();
        let contract_id = env.register(HarvestPayContract, ());
        let client = HarvestPayContractClient::new(&env, &contract_id);

        client.init(&buyer, &farmer, &amount);
        let state = client.get_state();

        assert_eq!(state.buyer, buyer);
        assert_eq!(state.farmer, farmer);
        assert_eq!(state.amount, 500_000_000);
        assert!(!state.deposited);
        assert!(!state.released);
    }

    /// Test 4 (Edge case): Re-initializing an already-initialized contract panics
    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_double_init_panics() {
        let (env, buyer, farmer, _token, amount) = setup();
        let contract_id = env.register(HarvestPayContract, ());
        let client = HarvestPayContractClient::new(&env, &contract_id);

        client.init(&buyer, &farmer, &amount);
        client.init(&buyer, &farmer, &amount); // should panic
    }

    /// Test 5 (Edge case): Releasing before depositing panics
    #[test]
    #[should_panic(expected = "funds not deposited")]
    fn test_release_before_deposit_panics() {
        let (env, buyer, farmer, token, amount) = setup();
        let contract_id = env.register(HarvestPayContract, ());
        let client = HarvestPayContractClient::new(&env, &contract_id);

        client.init(&buyer, &farmer, &amount);
        client.release(&buyer, &token); // no deposit yet — should panic
    }
}
