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
        vault: Vault,
    }

    impl AirTagNft {
        pub fn instantiate_component() -> Global<AirTagNft> {
            let air_tag_bucket: Bucket = ResourceBuilder::new_ruid_non_fungible(OwnerRole::None)
                .metadata(metadata! {
                  init {
                    "name" => "Real world asset - Air Tag", locked;
                    "symbol" => "AIRTAG", locked;
                  }
                })
                .mint_initial_supply([AirTag {
                    name: "Air tag 1".into(),
                    description: "Mariana".into(),
                    serial: "SERIAL123".into(),
                }])
                .into();

            Self {
                vault: Vault::with_bucket(air_tag_bucket),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
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

        pub fn take(&mut self) -> Bucket {
            info!(
                "My balance is: {} air tags tokens. There are not more air tags available.",
                self.vault.amount()
            );

            self.vault.take(1)
        }
    }
}
