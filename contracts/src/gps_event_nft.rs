use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct GpsEvent {
    latitude: i64,
    longitude: i64,
    timestamp: u64,
    air_tag_id: u64,
}

#[blueprint]
mod gps_event_nft {
    struct GpsEventNft {
        gps_event_resource_manager: ResourceManager,
        gps_event_id_counter: u64,
    }

    impl GpsEventNft {
        pub fn instantiate_component() -> Global<GpsEventNft> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(GpsEventNft::blueprint_id());

            let gps_event_resource_manager =
                ResourceBuilder::new_integer_non_fungible::<GpsEvent>(OwnerRole::None)
                    .metadata(metadata! {
                      init {
                        "name" => "Real world asset - Air Tag GPS Event", locked;
                        "symbol" => "GPS", locked;
                      }
                    })
                    .mint_roles(mint_roles!(
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(deny_all);
                    ))
                    .create_with_no_initial_supply();

            Self {
                gps_event_resource_manager,
                gps_event_id_counter: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }
    }
}
