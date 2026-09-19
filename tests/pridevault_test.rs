use multiversx_sc_scenario::imports::*;

const WASM_PATH: &str = "output/pridevault.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("");
    blockchain.register_contract(WASM_PATH, pridevault::ContractBuilder);
    blockchain
}

#[test]
fn empty_world_compiles() {
    let _ = world();
}
