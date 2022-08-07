// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASIC_PRODUCTIVITY :u32 = 221;
const SUCCESS_RATE_MAX: f64 = 1.0;
const SUCCESS_RATE_MODERATE: f64 = 0.9;
const SUCCESS_RATE_LOW: f64 = 0.77;

fn get_success_rate(speed: u8) -> f64 {
    match speed {
       0..=4 => SUCCESS_RATE_MAX,
       5..=8 => SUCCESS_RATE_MODERATE,
       9..=10 => SUCCESS_RATE_LOW,
       default => panic!("Unknown speed: {}", speed),
    }
}




pub fn production_rate_per_hour(speed: u8) -> f64 {
    (BASIC_PRODUCTIVITY as f64) * (speed as f64) * get_success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed)  / 60f64) as u32
}
