use tari_template_lib::prelude::*;

#[template]
mod my_nft_template {
    use super::*;

    pub struct MyNft {
        address: ResourceAddress,
        vault: Vault,
    }

    impl MyNft {
        pub fn create() -> (Component<Self>, ResourceAddress) {
            let nft_ids = [
                NonFungibleId::from_u32(1),
                NonFungibleId::from_string("Genesis"),
                NonFungibleId::from_u64(1000),
            ];

            // NUOVA API: allocazione indirizzo per Tari v0.9.0
            let allocation = CallerContext::allocate_address(args::SubstateType::Resource, None)
                .as_resource_address_allocation()
                .expect("Impossibile allocare un indirizzo per la risorsa NFT");
            let address = allocation.address().clone();

            let bucket = ResourceBuilder::non_fungible()
                .with_token_symbol("MYNFT")
                .with_address_allocation(allocation)
                .with_metadata(metadata! {
                    "name" => "My NFT Collection",
                    "description" => "Una collezione di NFT su Tari",
                    "image_url" => "https://example.com/nft.png",
                    "creator" => "MyZubster",
                })
                .mintable(rule!(allow_all))
                .burnable(rule!(deny_all))
                .initial_supply(nft_ids);

            let component = Component::new(Self {
                address: bucket.resource_address(),
                vault: Vault::from_bucket(bucket),
            })
            .with_access_rules(AccessRules::allow_all())
            .create();

            (component, address)
        }

        pub fn mint(&mut self, nft_id: NonFungibleId) -> Bucket {
            assert!(
                !self.vault.has_non_fungible(&nft_id), 
                "NFT già esistente"
            );

            let minted = ResourceManager::get(self.address)
                .mint_non_fungible(nft_id)
                .expect("Errore nel minting dell'NFT");

            self.vault.deposit(minted);
            self.vault.withdraw_non_fungible(&nft_id)
        }

        pub fn get_nft(&self, nft_id: NonFungibleId) -> Option<NonFungible> {
            self.vault.get_non_fungible(&nft_id)
        }

        pub fn get_all_nfts(&self) -> Vec<NonFungible> {
            self.vault.get_all_non_fungibles()
        }

        pub fn transfer(&mut self, nft_id: NonFungibleId, recipient: ComponentAddress) -> Bucket {
            let bucket = self.vault.withdraw_non_fungible(&nft_id);
            
            // Invia al destinatario
            recipient.deposit(bucket);
            
            // Restituisci il bucket per conferma
            self.vault.withdraw_non_fungible(&nft_id)
        }
    }
}
