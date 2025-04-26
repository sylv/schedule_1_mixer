use crate::game_data::{Drug, Effect, EffectId, GameData, Ingredient};

#[derive(Clone)]
pub struct EffectList(usize);

impl EffectList {
    pub fn new() -> Self {
        EffectList(0)
    }

    pub fn add_effect(&mut self, effect_id: &EffectId) {
        debug_assert!(effect_id < &64);
        self.0 |= 1 << effect_id;
    }

    pub fn remove_effect(&mut self, effect_id: &EffectId) {
        self.0 &= !(1 << effect_id);
    }

    pub fn has_effect(&self, effect_id: &EffectId) -> bool {
        debug_assert!(effect_id < &64);
        (self.0 & (1 << effect_id)) != 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn to_vec<'a>(&self, game_data: &'a GameData) -> Vec<&'a Effect> {
        let mut effects = Vec::new();
        for effect in game_data.effects.values() {
            if self.has_effect(&effect.id) {
                effects.push(effect);
            }
        }

        effects
    }

    pub fn from_effects(effects: Vec<&EffectId>) -> Self {
        let mut effect_list = EffectList::new();
        for effect in effects {
            effect_list.add_effect(effect);
        }
        effect_list
    }
}

pub fn mix(base_drug: &Drug, ingredients: &[&Ingredient]) -> EffectList {
    let mut current_effects = EffectList::new();
    if let Some(base_effect_id) = base_drug.base_effect_id {
        current_effects.add_effect(&base_effect_id);
    }

    for ingredient in ingredients {
        let mut new_effects = current_effects.clone();
        let mut delayed_replacers = Vec::new();
        for (from_effect, to_effect) in &ingredient.replaces_effects {
            if current_effects.has_effect(&from_effect) {
                if current_effects.has_effect(&to_effect) {
                    delayed_replacers.push((from_effect, to_effect));
                } else {
                    new_effects.remove_effect(&from_effect);
                    new_effects.add_effect(&to_effect);
                }
            }
        }

        if new_effects.len() < 8 {
            new_effects.add_effect(&ingredient.adds_effect);
        }

        for (from_effect, to_effect) in delayed_replacers {
            if new_effects.has_effect(from_effect) {
                if !new_effects.has_effect(to_effect) {
                    new_effects.remove_effect(from_effect);
                    new_effects.add_effect(to_effect);
                }
            }
        }

        current_effects = new_effects;
    }

    current_effects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_data::{GameData, Ingredient};

    fn test_mix_helper(
        base_drug_name: &str,
        ingredient_names: Vec<&str>,
        expected_effects: Vec<&str>,
    ) {
        let game_data = GameData::load().unwrap();
        let base_drug = game_data.get_drug_by_name(base_drug_name).unwrap();
        let ingredients: Vec<&Ingredient> = ingredient_names
            .iter()
            .map(|&name| game_data.get_ingredient_by_name(name).unwrap())
            .collect();

        let pred_effects = mix(base_drug, &ingredients);
        let pred_effects: Vec<String> = pred_effects
            .to_vec(&game_data)
            .into_iter()
            .map(|effect| effect.name.clone())
            .collect();

        assert_eq!(pred_effects.len(), expected_effects.len());
        for effect in expected_effects {
            assert!(pred_effects.contains(&effect.to_string()));
        }
    }

    #[test]
    fn test_mix_1() {
        test_mix_helper(
            "Methamphetamine",
            vec!["Gasoline", "Cuke", "Mouth Wash", "Banana"],
            vec!["Balding", "Euphoric", "Gingeritis", "Thought-Provoking"],
        );
    }

    #[test]
    fn test_mix_2() {
        test_mix_helper(
            "Green Crack",
            vec!["Cuke", "Iodine", "Mega Bean", "Energy Drink", "Horse Semen"],
            vec![
                "Athletic",
                "Cyclopean",
                "Laxative",
                "Long Faced",
                "Paranoia",
            ],
        );
    }

    #[test]
    fn test_mix_3() {
        test_mix_helper(
            "OG Kush",
            vec!["Donut", "Donut", "Addy", "Battery"],
            vec![
                "Bright-Eyed",
                "Calming",
                "Calorie-Dense",
                "Thought-Provoking",
                "Zombifying",
            ],
        );
    }

    #[test]
    fn test_mix_4() {
        test_mix_helper(
            "OG Kush",
            vec!["Donut", "Battery", "Flu Medicine", "Addy", "Horse Semen"],
            vec![
                "Bright-Eyed",
                "Calming",
                "Calorie-Dense",
                "Electrifying",
                "Long Faced",
                "Refreshing",
            ],
        )
    }

    #[test]
    fn test_mix_5() {
        test_mix_helper(
            "Granddaddy Purple",
            vec![
                "Flu Medicine",
                "Cuke",
                "Addy",
                "Battery",
                "Mega Bean",
                "Horse Semen",
                "Viagor",
                "Paracetamol",
            ],
            vec![
                "Bright-Eyed",
                "Calming",
                "Cyclopean",
                "Long Faced",
                "Paranoia",
                "Refreshing",
                "Sneaky",
                "Tropic Thunder",
            ],
        )
    }
}
