use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct BacdiveSpeciesJson {
    pub id: String,
    pub species: String,
    pub strain: String,
    pub information: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BacdiveSearchSpecies {
    pub id: String,
    pub species: String,
    pub speciesinformation: String,
}
