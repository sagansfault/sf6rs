# sf6rs
A rust library for interfacing with various types of Street Fighter 6 data like frame data via web-scraping [SuperCombo.gg](https://wiki.supercombo.gg/w/Street_Fighter_6).

## Frame Data
This library requests, scrapes, parses, and collects character frame data:
```rust
// Loading all data
let data = framedata::load_all().await;

// Loading moves
let load = framedata::load(&character::RYU).await;

// Searching for moves
let move_found = data.find_move("ryu", "5lp");
let another_move_found = data.find_move_character(&character::RYU, "623HP");
```