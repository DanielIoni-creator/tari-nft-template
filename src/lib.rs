use tari_template_abi::*;
use serde::{Deserialize, Serialize};

// ============================================
// METADATI DELLA PIANTA
// ============================================
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlantMetadata {
    pub name: String,
    pub species: String,
    pub age_years: u32,
    pub price_in_xmr: f64,
    pub image_hash: String,
}

// ============================================
// STRUTTURA DELL'NFT PIANTA
// ============================================
#[derive(Clone, Debug)]
pub struct PlantNFT {
    pub id: u64,
    pub owner: [u8; 32], // Address come array di 32 byte
    pub metadata: PlantMetadata,
}

// ============================================
// FUNZIONI DEL TEMPLATE
// ============================================
pub fn mint_plant(
    owner: [u8; 32],
    name: String,
    species: String,
    age_years: u32,
    price_in_xmr: f64,
    image_hash: String,
) -> PlantNFT {
    let id = 1; // ID univoco (in produzione usa un generatore)
    let metadata = PlantMetadata {
        name,
        species,
        age_years,
        price_in_xmr,
        image_hash,
    };
    PlantNFT { id, owner, metadata }
}

pub fn transfer_plant(
    plant: PlantNFT,
    new_owner: [u8; 32],
) -> PlantNFT {
    PlantNFT {
        owner: new_owner,
        ..plant
    }
}

pub fn get_plant_metadata(plant: &PlantNFT) -> PlantMetadata {
    plant.metadata.clone()
}

pub fn get_plant_owner(plant: &PlantNFT) -> [u8; 32] {
    plant.owner
}
