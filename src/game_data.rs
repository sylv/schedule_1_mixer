use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashMap, fs};

pub type EffectId = u8;
pub type DrugId = u8;
pub type IngredientId = u8;

#[derive(Debug, Deserialize)]
pub struct Rank {
    pub name: String,
    pub level: u8,
}

#[derive(Debug, Deserialize)]
pub struct Effect {
    pub id: EffectId,
    pub name: String,
    pub description: Option<String>,
    pub is_ability: bool,
    pub is_cosmetic: bool,
    pub multiplier: f32,
}

#[derive(Debug, Deserialize)]
pub struct Drug {
    pub id: DrugId,
    pub name: String,
    pub base_effect_id: Option<EffectId>,
    pub base_sell_value: u8,
    pub unlock_rank: u8,
}

#[derive(Debug, Deserialize)]
pub struct Ingredient {
    pub id: IngredientId,
    pub name: String,
    pub buy_price: u8,
    pub unlock_rank: u8,
    pub adds_effect: EffectId,
    pub replaces_effects: HashMap<EffectId, EffectId>,
}

#[derive(Debug, Deserialize)]
pub struct GameData {
    pub ranks: Vec<Rank>,
    pub effects: HashMap<EffectId, Effect>,
    pub drugs: HashMap<DrugId, Drug>,
    pub ingredients: HashMap<IngredientId, Ingredient>,
}

impl GameData {
    pub fn load() -> Result<Self> {
        let file_content =
            fs::read_to_string("data/data.json").context("Failed to read game data file")?;
        let game_data: GameData =
            serde_json::from_str(&file_content).context("Failed to parse game data JSON")?;
        Ok(game_data)
    }

    pub fn get_effect(&self, id: EffectId) -> Option<&Effect> {
        self.effects.get(&id)
    }

    pub fn get_effect_by_name(&self, name: &str) -> Option<&Effect> {
        self.effects.values().find(|effect| effect.name == name)
    }

    pub fn get_drug(&self, id: DrugId) -> Option<&Drug> {
        self.drugs.get(&id)
    }

    pub fn get_drug_by_name(&self, name: &str) -> Option<&Drug> {
        self.drugs.values().find(|drug| drug.name == name)
    }

    pub fn get_ingredient(&self, id: IngredientId) -> Option<&Ingredient> {
        self.ingredients.get(&id)
    }

    pub fn get_ingredient_by_name(&self, name: &str) -> Option<&Ingredient> {
        self.ingredients
            .values()
            .find(|ingredient| ingredient.name == name)
    }
}
