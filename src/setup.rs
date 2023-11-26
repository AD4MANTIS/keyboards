use once_cell::sync::Lazy;

use crate::models::key_maps::{get_key_map_dict_qwertz, KeyMapDict};

// rng
pub const SEED: u64 = 123456;

// data
pub const BOOK_PATH: &str = "resources/meinBuch.txt";
pub const TEMPERATURE: f64 = 500.0;
pub const EPOCH: f64 = 20.0;
pub const COOLING_RATE: f64 = 0.99;
pub const NUM_ITERATIONS: usize = 5000;

pub static KEY_MAP_DICT: Lazy<KeyMapDict> = Lazy::new(get_key_map_dict_qwertz);

// weights
pub const DISTANCE_EFFORT: u32 = 1; // at 2 distance penalty is squared
pub const DOUBLE_FINGER_EFFORT: i32 = 1;
pub const DOUBLE_HAND_EFFORT: i32 = 1;
