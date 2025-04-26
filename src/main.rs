use filter::Filter;
use game_data::{Drug, Effect, GameData, Ingredient};
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use mix::mix;
use rayon::prelude::*;
use std::{cmp::Ordering, time::Instant};

mod filter;
mod game_data;
mod mix;

#[derive(Debug)]
pub struct MixResult<'a> {
    base_drug: &'a Drug,
    ingredients: Vec<&'a Ingredient>,
    cost: u32,
    sell_price: u32,
    profit: u32,
    effects: Vec<&'a Effect>,
    multiplier: f32,
}

fn find_optimal_mix<'a>(filter: &Filter, game_data: &'a GameData) -> Option<MixResult<'a>> {
    println!("Starting search with filter: {:#?}", filter);

    let start_time = Instant::now();
    let total_combinations: usize = filter.available_drugs.len()
        * (1..=filter.max_ingredients)
            .map(|k| filter.available_ingredients.len().pow(k as u32))
            .sum::<usize>();

    println!(
        "Testing {} possible combinations across all strains...",
        total_combinations
    );

    let progress_bar = indicatif::ProgressBar::new(total_combinations as u64);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%, {eta}) {per_sec}",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let best_option = filter
        .available_drugs
        .par_iter()
        .flat_map(|drug_id| {
            (1..=filter.max_ingredients)
                .into_par_iter()
                .flat_map(|ingredient_count| {
                    // Generate all combinations with repetition allowed
                    (0..ingredient_count)
                        .map(|_| filter.available_ingredients.iter().cloned())
                        .multi_cartesian_product()
                        .par_bridge()
                        .progress_with(progress_bar.clone())
                        .filter_map(|ingredient_ids| {
                            // Rest of your filtering logic remains the same
                            let ingredients = ingredient_ids
                                .iter()
                                .map(|id| game_data.get_ingredient(*id).unwrap())
                                .collect::<Vec<_>>();

                            let base_drug = game_data.get_drug(*drug_id).unwrap();
                            let effects = mix(base_drug, &ingredients);

                            for required_effect in &filter.required_effects {
                                if !effects.has_effect(required_effect) {
                                    return None;
                                }
                            }

                            for blocked_effect in &filter.blocked_effects {
                                if effects.has_effect(blocked_effect) {
                                    return None;
                                }
                            }

                            let effects = effects.to_vec(&game_data);
                            let cost = ingredients.iter().map(|i| i.buy_price as u32).sum::<u32>();
                            let multiplier: f32 =
                                effects.iter().map(|effect| effect.multiplier).sum();
                            let sell_price = (base_drug.base_sell_value as f32 * (1.0 + multiplier))
                                .round() as u32;
                            let profit = sell_price.saturating_sub(cost);

                            Some(MixResult {
                                base_drug,
                                ingredients,
                                cost,
                                sell_price,
                                profit,
                                effects,
                                multiplier,
                            })
                        })
                })
        })
        .reduce_with(|existing_result, new_result| {
            for target in &filter.optimize_targets {
                match target.compare(&existing_result, &new_result) {
                    Ordering::Less => return new_result,
                    Ordering::Greater => return existing_result,
                    Ordering::Equal => {}
                }
            }

            existing_result
        });

    progress_bar.finish_with_message("Search completed for all strains!");

    let duration = start_time.elapsed();
    println!("Total search completed in {:.2?}", duration);

    best_option
}

fn main() -> anyhow::Result<()> {
    let game_data = GameData::load().unwrap();
    let mut filter = Filter::new(&game_data);
    let filter = &mut filter
        .add_all_drugs()
        .add_all_ingredients()
        .with_max_ingredients(8);

    println!("Starting search...");
    let result = find_optimal_mix(&filter, &game_data);
    match result {
        Some(mix_result) => {
            println!("Best mix found:");
            println!("Base drug: {}", mix_result.base_drug.name);
            println!(
                "Ingredients: {:?}",
                mix_result
                    .ingredients
                    .iter()
                    .map(|i| i.name.clone())
                    .collect::<Vec<_>>()
            );
            println!("Cost: {}", mix_result.cost);
            println!("Sell price: {}", mix_result.sell_price);
            println!("Profit: {}", mix_result.profit);
            println!(
                "Effects: {:?}",
                mix_result
                    .effects
                    .iter()
                    .map(|e| e.name.clone())
                    .collect::<Vec<_>>()
            );
            println!("Multiplier: {:.2}", mix_result.multiplier);
        }
        None => {
            println!("No valid mix found.");
        }
    }

    Ok(())
}
