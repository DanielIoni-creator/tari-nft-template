// Copyright 2026 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use tari_template_lib::prelude::*;

/// Template per la creazione di NFT su Tari.
/// Questo template permette di:
/// - Creare una collezione di NFT
/// - Mintare nuovi NFT con metadati personalizzati
#[template]
mod my_first_nft {
    use super::*;

    /// Componente principale del template NFT.
    pub struct MyFirstNFT {
        /// Vault che contiene gli NFT di questa collezione
        nft_vault: Vault,
        /// Nome della collezione
        collection_name: String,
        /// Simbolo della collezione
        collection_symbol: String,
        /// Contatore degli NFT mintati
        mint_count: u32,
    }

    impl MyFirstNFT {
        /// Costruttore: crea una nuova collezione NFT.
        ///
        /// # Arguments
        /// * `name` - Nome della collezione (es. "My Art Collection")
        /// * `symbol` - Simbolo della collezione (es. "MAC")
        pub fn new(name: String, symbol: String) -> Component<Self> {
            // 1. Creiamo la risorsa NFT (la collezione)
            let nft_resource = ResourceBuilder::non_fungible()
                .metadata("name", name.clone())
                .metadata("symbol", symbol.clone())
                .metadata("description", "Una collezione NFT su Tari")
                .build();

            // 2. Creiamo la vault vuota che conterrà gli NFT
            let vault = Vault::new_empty(nft_resource);

            // 3. Definiamo le regole di accesso
            let access_rules = ComponentAccessRules::new()
                .method("mint", rule![allow_all]);

            // 4. Creiamo il componente
            Component::new(Self {
                nft_vault: vault,
                collection_name: name,
                collection_symbol: symbol,
                mint_count: 0,
            })
            .with_access_rules(access_rules)
            .create()
        }

        /// Mint di un nuovo NFT.
        ///
        /// # Arguments
        /// * `id` - Identificatore univoco dell'NFT (es. "nft-001")
        /// * `immutable_data` - Dati immutabili (es. URL dell'immagine)
        /// * `mutable_data` - Dati mutabili (es. stato corrente)
        ///
        /// # Returns
        /// Un Bucket contenente l'NFT appena mintato.
        pub fn mint(
            &mut self,
            id: NonFungibleId,
            immutable_data: String,
            mutable_data: String,
        ) -> Bucket {
            // Otteniamo il ResourceManager per la risorsa NFT
            let manager = self.nft_vault.get_resource_manager();

            // Cloniamo l'ID perché lo useremo due volte
            let id_clone = id.clone();

            // Mintiamo l'NFT con i dati forniti
            let nft_bucket = manager.mint_non_fungible(
                id_clone,
                &metadata! {
                    "immutable" => immutable_data,
                    "minted_at" => "2026-07-19",
                },
                &mutable_data,
            );

            // Depositiamo l'NFT nella vault
            self.nft_vault.deposit(nft_bucket);

            // Incrementiamo il contatore
            self.mint_count += 1;

            // Restituiamo l'NFT come Bucket
            self.nft_vault.withdraw_non_fungible(id)
        }
    }
}