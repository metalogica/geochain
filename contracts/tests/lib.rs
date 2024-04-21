use radix_engine_interface::prelude::*;
use scrypto::this_package;
use scrypto_test::prelude::*;
use scrypto_unit::*;

use geochain::test_bindings::AirTagNft;

#[test]
fn test_air_tag_nft() {
    let mut test_runner = TestRunnerBuilder::new().build();

    let (public_key, _private_key, _account) = test_runner.new_allocated_account();

    let package_address = test_runner.compile_and_publish(this_package!());

    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "AirTagNft",
            "instantiate",
            manifest_args!(),
        )
        .build();

    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    receipt.expect_commit(true).new_component_addresses()[0];
}

#[test]
fn test_ait_tag_with_test_environment() -> Result<(), RuntimeError> {
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    let mut air_tag_nft = AirTagNft::instantiate(package_address, &mut env)?;

    let _ = air_tag_nft.mint(
        "my-air-tag".into(),
        "super-useful-description".into(),
        "SERIAL1".into(),
        &mut env,
    );

    let my_air_tag = air_tag_nft.get(0, &mut env);

    assert_eq!(my_air_tag.unwrap().name, "my-air-tag");

    Ok(())
}
