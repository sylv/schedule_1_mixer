import { s } from "@ryanke/spider";
import assert from "node:assert";
import { writeFile } from "node:fs/promises";

type EffectId = number;
type IngredientId = number;
type DrugId = number;

// Hoodlum IV
interface Rank {
  name: string;
  level: number;
}

// OG Kush
interface Drug {
  name: string;
  id: DrugId;
  base_effect_id: EffectId | null;
  base_sell_value: number;
  unlock_rank: number;
}

// Anti-Gravity
interface Effect {
  name: string;
  id: EffectId;
  multiplier: number;
  is_ability: boolean;
  is_cosmetic: boolean;
  description: string | null;
}

// Cuke
interface Ingredient {
  name: string;
  id: IngredientId;
  buy_price: number;
  unlock_rank: number | null;
  adds_effect: EffectId;
  replaces_effects: Record<EffectId, EffectId>;
}

const extractRanks = async (): Promise<Rank[]> => {
  const schema = s([
    "table.wikitable tbody:contains(Street Rat I) tr:has(td)",
    s({
      name: s("td:nth-child(1)").trim(),
      total_xp: s("td:nth-child(3)").number(),
    }),
  ]);

  const html = await fetch("https://schedule-1.fandom.com/wiki/Ranks").then((res) => res.text());
  const data = schema.parseHTML(html);
  data.sort((a, b) => a.total_xp - b.total_xp);

  assert(data.length > 0, "No ranks found");
  return data.map((rank, index) => ({
    name: rank.name,
    level: index + 1,
  }));
};

const extractEffects = async (): Promise<Effect[]> => {
  const schema = s([
    "table.wikitable tbody:contains(Causes user to jump higher) tr:has(td)",
    s({
      name: s("td:nth-child(1) span[style]").trim(),
      multiplier: s("td:nth-child(3)").number(),
      type: s("td:nth-child(4)").trim().optional(),
      description: s("td:nth-child(5)").trim().optional(),
    }),
  ]);

  const html = await fetch("https://schedule-1.fandom.com/wiki/Effects").then((res) => res.text());
  const data = schema.parseHTML(html);

  assert(data.length > 0, "No effects found");
  return data.map((effect, index) => {
    return {
      name: effect.name,
      id: index + 1,
      description: effect.description,
      is_ability: effect.type === "Ability",
      is_cosmetic: effect.type === "Cosmetic",
      multiplier: effect.multiplier,
    };
  });
};

const getEffectByName = (effects: Effect[], name: string): Effect => {
  const effect = effects.find((effect) => effect.name === name);
  if (!effect) {
    throw new Error(`Effect ${name} not found`);
  }

  return effect;
};

const getRankByName = (ranks: Rank[], name: string): Rank => {
  const rank = ranks.find((rank) => rank.name.toLowerCase() === name.toLowerCase());
  if (!rank) {
    throw new Error(`Rank ${name} not found`);
  }

  return rank;
};

const extractDrugs = async (effects: Effect[], ranks: Rank[]): Promise<Drug[]> => {
  const schema = s([
    "table.fandom-table tbody:contains(Street Rat I) tr:has(td)",
    s({
      name: s("td:nth-child(1)").trim(),
      rank: s("td:nth-child(3)").trim(),
      base_sell_value: s("td:nth-child(5)").number(),
      base_effect: s("td:nth-child(6)").trim(),
    }),
  ]);

  const html = await fetch("https://schedule-1.fandom.com/wiki/Drugs").then((res) => res.text());
  const data = schema.parseHTML(html);

  assert(data.length > 0, "No drugs found");
  return data.map((drug, index) => {
    const rank = getRankByName(ranks, drug.rank);
    if (drug.base_effect === "None") {
      return {
        name: drug.name,
        id: index + 1,
        base_effect_id: null,
        base_sell_value: drug.base_sell_value,
        unlock_rank: rank.level,
      };
    }

    const effect = getEffectByName(effects, drug.base_effect);
    return {
      name: drug.name,
      id: index + 1,
      base_effect_id: effect.id,
      base_sell_value: drug.base_sell_value,
      unlock_rank: rank.level,
    };
  });
};

const extractIngredients = async (effects: Effect[], ranks: Rank[]): Promise<Ingredient[]> => {
  const schema = s([
    "table.fandom-table tbody:contains(A refreshing can of Cuke) tr:has(td)",
    s({
      name: s("td:nth-child(2)").trim(),
      buy_price: s("td:nth-child(3)").number(),
      rank: s("td:nth-child(4)").trim(),
      base_effect: s("td:nth-child(6)").trim(),
    }),
  ]);

  const html = await fetch("https://schedule-1.fandom.com/wiki/Ingredients").then((res) => res.text());
  const data = schema.parseHTML(html);

  assert(data.length > 0, "No ingredients found");
  return data.map((ingredient, index) => {
    const rank = getRankByName(ranks, ingredient.rank);
    const effect = getEffectByName(effects, ingredient.base_effect);
    return {
      name: ingredient.name,
      id: index + 1,
      buy_price: ingredient.buy_price,
      unlock_rank: rank.level,
      adds_effect: effect.id,
      replaces_effects: {},
    };
  });
};

const extractRules = async (ingredients: Ingredient[]) => {
  const schema = s([
    "table.fandom-table tbody:contains(only when Paranoia isn't already in the mix) tr:has(td)",
    s({
      effect: s("td:nth-child(1) span[style]").trim(),
      product: s("td:nth-child(2)").trim(),
      replaces: s("td:nth-child(3) span[style]").trim().optional(),
    }),
  ]);

  const html = await fetch("https://schedule-1.fandom.com/wiki/Mixing").then((res) => res.text());
  const data = schema.parseHTML(html);

  assert(data.length > 0, "No rules found");
  for (const rule of data) {
    const product = ingredients.find((ingredient) => ingredient.name === rule.product);
    if (!product) {
      throw new Error(`Product ${rule.product} not found`);
    }

    if (!rule.replaces) {
      // todo: check if effect matches base effect, which i think is why these entries exist
      continue;
    }

    const effect = getEffectByName(effects, rule.effect);
    const replacesEffect = getEffectByName(effects, rule.replaces);
    product.replaces_effects[replacesEffect.id] = effect.id;
  }
};

const ranks = await extractRanks();
const effects = await extractEffects();
const drugs = await extractDrugs(effects, ranks);
const ingredients = await extractIngredients(effects, ranks);
await extractRules(ingredients);

const data = {
  ranks,
  effects: {},
  drugs: {},
  ingredients: {},
};

for (const effect of effects) data.effects[effect.id] = effect;
for (const drug of drugs) data.drugs[drug.id] = drug;
for (const ingredient of ingredients) data.ingredients[ingredient.id] = ingredient;

const json = JSON.stringify(data, null, 2);
await writeFile("data.json", json, "utf-8");
