# schedule_1_mixer

Finds the optimal mix of ingredients for a target in [Schedule 1](https://store.steampowered.com/app/3164500/Schedule_I/)

Can check around 20m combinations/sec, or the whole 27 billion(?) possible combinations in about 22 minutes

Will find the optimal result and can filter for lowest cost, highest cost, specific base, specific ingredients, required/blocked effects etc


I was going to make this a website but wasm_bindgen sucks and I can't be bothered

## notes

- `data/` contains the code I used to scrape the game data from the wiki
- `src/mix.rs` is what simulates the mixes, it seems to line up with the game and other tools
- Always searches 1-8 ingredients (unless max ingredients is lower), and reuses ingredients which I think is correct
- I haven't fully verified that the data is correct, it might end up being wrong in some situations