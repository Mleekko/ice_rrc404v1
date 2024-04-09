use scrypto::this_package;
use scrypto_unit::*;
use transaction::prelude::*;

pub const RRC404_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
        13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 225, 206, 28, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ]); // package_sim1p5qqqqqqqqqqqqqqqqqpecwwrnsqqqqqqqqqqqqqqqqqqqqqj5zvnh

pub const RRC404_COMPONENT: ComponentAddress = ComponentAddress::new_or_panic([
    192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 225, 206, 28, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
]); // component_sim1cqqqqqqqqqqqqqqqqqqpecwwrnsqqqqqqqqqqqqqqqqqqqqqgguvvr


#[test]
fn test_instantiate() {
    let mut test_runner = TestRunnerBuilder::new().build();

    let (public_key, _, owner_account) = test_runner.new_allocated_account();

    test_runner.compile_and_publish_at_address(this_package!(), RRC404_PACKAGE);

    let receipt = test_runner.execute_system_transaction_with_preallocated_addresses(
        vec![
            InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(RRC404_PACKAGE),
                blueprint_name: "Rrc404".to_string(),
                function_name: "instantiate".to_string(),
                args: manifest_args!(Some(ManifestAddressReservation(0))).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(owner_account.into())),
                method_name: "deposit_batch".to_string(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            }],
        vec![(
                 BlueprintId::new(&RRC404_PACKAGE, "Rrc404"),
                 GlobalAddress::new_or_panic(RRC404_COMPONENT.into()),
             )
                 .into()],
        btreeset!(NonFungibleGlobalId::from_public_key(&public_key)),
    );

    let result = receipt.expect_commit_success();
    let rrc404_component = result.new_component_addresses()[0];

    println!("rrc404_component: {:?}", rrc404_component)
}
