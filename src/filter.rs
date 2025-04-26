use crate::{
    MixResult,
    game_data::{DrugId, EffectId, GameData, IngredientId},
};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum OptimizeTarget {
    Profit,
    SellPrice,
    Cost,
    FewestEffects,
    MostEffects,
    Multiplier,
    Ingredients,
}

impl OptimizeTarget {
    pub fn compare(&self, a: &MixResult, b: &MixResult) -> std::cmp::Ordering {
        match self {
            OptimizeTarget::Profit => a.profit.cmp(&b.profit),
            OptimizeTarget::SellPrice => a.sell_price.cmp(&b.sell_price),
            OptimizeTarget::Cost => b.cost.cmp(&a.cost), // Reversed
            OptimizeTarget::FewestEffects => b.effects.len().cmp(&a.effects.len()), // Reversed
            OptimizeTarget::MostEffects => b.effects.len().cmp(&a.effects.len()),
            OptimizeTarget::Multiplier => a
                .multiplier
                .partial_cmp(&b.multiplier)
                .unwrap_or(Ordering::Equal),
            OptimizeTarget::Ingredients => b.ingredients.len().cmp(&a.ingredients.len()), // Reversed
        }
    }
}

#[derive(Debug)]
pub struct Filter<'a> {
    pub optimize_targets: Vec<OptimizeTarget>,
    pub available_drugs: Vec<DrugId>,
    pub available_ingredients: Vec<IngredientId>,
    pub required_effects: Vec<EffectId>,
    pub blocked_effects: Vec<EffectId>,
    pub max_ingredients: usize,
    game_data: &'a GameData,
}

#[allow(dead_code)]
impl<'a> Filter<'a> {
    pub fn new(game_data: &'a GameData) -> Self {
        Self {
            optimize_targets: vec![
                OptimizeTarget::Profit,
                OptimizeTarget::Cost,
                OptimizeTarget::Ingredients,
            ],
            available_drugs: Vec::new(),
            available_ingredients: Vec::new(),
            required_effects: Vec::new(),
            blocked_effects: Vec::new(),
            max_ingredients: 8,
            game_data,
        }
    }

    pub fn with_optimize_targets(&mut self, target: Vec<OptimizeTarget>) -> &mut Self {
        self.optimize_targets = target;
        self
    }

    pub fn add_drug(&mut self, drug_name: &str) -> &mut Self {
        let drug = self
            .game_data
            .get_drug_by_name(drug_name)
            .unwrap_or_else(|| panic!("Strain '{}' not found in game data", drug_name));

        if !self.available_drugs.contains(&drug.id) {
            self.available_drugs.push(drug.id);
        }
        self
    }

    pub fn add_all_drugs(&mut self) -> &mut Self {
        self.available_drugs = self.game_data.drugs.values().map(|s| s.id).collect();
        self
    }

    pub fn add_ingredient(&mut self, ingredient_name: &str) -> &mut Self {
        let ingredient = self
            .game_data
            .get_ingredient_by_name(ingredient_name)
            .unwrap_or_else(|| panic!("Ingredient '{}' not found in game data", ingredient_name));

        if !self.available_ingredients.contains(&ingredient.id) {
            self.available_ingredients.push(ingredient.id);
        }
        self
    }

    pub fn add_all_ingredients(&mut self) -> &mut Self {
        self.available_ingredients = self.game_data.ingredients.values().map(|s| s.id).collect();
        self
    }

    pub fn add_required_effect(&mut self, effect_name: &str) -> &mut Self {
        let effect = self
            .game_data
            .get_effect_by_name(effect_name)
            .unwrap_or_else(|| panic!("Effect '{}' not found in game data", effect_name));

        if !self.required_effects.contains(&effect.id) {
            self.required_effects.push(effect.id);
        }
        self
    }

    pub fn add_blocked_effect(&mut self, effect_name: &str) -> &mut Self {
        let effect = self
            .game_data
            .get_effect_by_name(effect_name)
            .unwrap_or_else(|| panic!("Effect '{}' not found in game data", effect_name));

        if !self.blocked_effects.contains(&effect.id) {
            self.blocked_effects.push(effect.id);
        }
        self
    }

    pub fn with_max_ingredients(&mut self, max_ingredients: usize) -> &mut Self {
        self.max_ingredients = max_ingredients;
        self
    }
}
