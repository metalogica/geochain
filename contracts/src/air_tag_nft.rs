use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct GpsEvent {
    latitude_degree: i8,
    latitude_decimal: u64,
    longitude_degree: i8,
    longitude_decimal: u64,
    timestamp: Instant,
}

#[derive(NonFungibleData, ScryptoSbor)]
pub struct AirTag {
    name: String,
    description: String,
    serial: String,
    gps_events: Vec<GpsEvent>,
}

#[blueprint]
mod air_tag_nft {
    struct AirTagNft {
        air_tag_id_counter: u64,
        air_tag_resource_manager: ResourceManager,
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
                gps_events: vec![GpsEvent {
                    latitude_degree: 1,
                    latitude_decimal: 1,
                    longitude_degree: 1,
                    longitude_decimal: 1,
                    timestamp: Clock::current_time_rounded_to_seconds(),
                }],
            };

            let new_air_tag_bucket = self.air_tag_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::Integer(self.air_tag_id_counter.into()),
                new_air_tag,
            );

            self.air_tag_id_counter += 1;

            new_air_tag_bucket
        }

        pub fn create_gps_event(&mut self) {
            // take air tag id
            // instantiate new gps event
        }
    }
}
