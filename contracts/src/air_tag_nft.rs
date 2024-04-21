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
        // vault: Vault,
        air_tag_resource_manager: ResourceManager,
        total_supply: u64,
    }

    impl AirTagNft {
        pub fn instantiate_component() -> Global<AirTagNft> {
            let air_tag_resource_manager =
                ResourceBuilder::new_integer_non_fungible::<AirTag>(OwnerRole::None)
                    .metadata(metadata! {
                      init {
                        "name" => "Real world asset - Air Tag", locked;
                        "symbol" => "AIRTAG", locked;
                      }
                    })
                    .create_with_no_initial_supply();

            Self {
                air_tag_resource_manager,
                total_supply: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn mint(&mut self) -> Bucket {
            let new_air_tag = AirTag {
                name: "my new air tag".into(),
                description: "I'm ADHD so I love things all the time!".into(),
                serial: "SERIALOUSLY?!".into(),
            };

            let new_air_tag_bucket = self.air_tag_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::Integer(self.total_supply.into()),
                new_air_tag,
            );

            self.total_supply += 1;

            new_air_tag_bucket
        }

        // pub fn mint(&mut self) -> Bucket {
        //     let new_air_tag = AirTag {
        //         name: "my new air tag",
        //         description: "I'm ADHD so I love things all the time!",
        //         serial: "SERIALOUSLY?!",
        //     };

        // let new_air_tag_bucket = ResourceBuilder::new_ruid_non_fungible(OwnerRole::None)
        //   .metadata(metadata! {
        //       init {
        //         "name" => "Real world asset - Air Tag", locked;
        //         "symbol" => "AIRTAG", locked;
        //       }
        //     })
        // }

        // pub fn take(&mut self) -> Bucket {
        //     info!(
        //         "My balance is: {} air tags tokens. There are not more air tags available.",
        //         self.vault.amount()
        //     );

        //     self.vault.take(1)
        // }
    }
}
