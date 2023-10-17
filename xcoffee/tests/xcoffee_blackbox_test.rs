use multiversx_sc_scenario::{api::StaticApi, scenario_model::*, *};
use xcoffee::*;

// Path the the output WASM file
const XCOFFEE_PATH_EXPR: &str = "file:output/xcoffee.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/xcoffee");

    blockchain.register_contract(XCOFFEE_PATH_EXPR, xcoffee::ContractBuilder);
    blockchain
}

#[test]
fn xcoffee_blackbox_test() {
    let mut world = world();

    let owner_address = "address:owner";
    let creator_address = "address:creator";

    let callee_user_address = AddressValue::from(creator_address).to_address();

    let mut xcoffee_contract = ContractInfo::<xcoffee::Proxy<StaticApi>>::new("sc:xcoffee");
    let xcoffee_code = world.code_expression(XCOFFEE_PATH_EXPR);

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:xcoffee"),
        )
        .sc_deploy_use_result(
            ScDeployStep::new()
                .from(owner_address)
                .code(xcoffee_code)
                .call(xcoffee_contract.init()),
            |new_address, _: TypedResponse<()>| {
                assert_eq!(new_address, xcoffee_contract.to_address());
            },
        )
        .sc_call(
            ScCallStep::new()
                .from(owner_address)
                .to(&xcoffee_contract)
                .call(xcoffee_contract.donate(callee_user_address, "Marius", "About me")),
        )
        .write_scenario_trace("scenarios/blackbox.scen.json");
}
