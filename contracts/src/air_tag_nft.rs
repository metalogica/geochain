use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct AirTag {
    name: String,
    description: String,
    serial: String,
}

#[blueprint]
mod air_tag_nft {
    struct AirTagNft {
        // TODO: a vault containing all GPS events for this air tag
        // gps_events: Vec<GpsEvent>,
        air_tag_resource_manager: ResourceManager,
        air_tag_id_counter: u64,
    }

    impl AirTagNft {
        pub fn instantiate_component() -> Global<AirTagNft> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(AirTagNft::blueprint_id());

            let air_tag_resource_manager =
                ResourceBuilder::new_integer_non_fungible::<AirTag>(OwnerRole::None)
                    .metadata(metadata! {
                      init {
                        "name" => "Real world asset - Air Tag", locked;
                        "symbol" => "AIRTAG", locked;
                      }
                    })
                    .mint_roles(mint_roles!(
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(deny_all);
                    ))
                    .burn_roles(burn_roles!(
                        burner => rule!(require(global_caller(component_address)));
                        burner_updater => rule!(deny_all);
                    ))
                    .create_with_no_initial_supply();

            Self {
                air_tag_resource_manager,
                air_tag_id_counter: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }

        pub fn mint(&mut self, name: String, description: String, serial: String) -> Bucket {
            let new_air_tag = AirTag {
                name,
                description,
                serial,
            };

            let new_air_tag_bucket = self.air_tag_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::Integer(self.air_tag_id_counter.into()),
                new_air_tag,
            );

            self.air_tag_id_counter += 1;

            new_air_tag_bucket
        }
    }
}
