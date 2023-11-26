// #![feature(test)]

use chrono::Utc;
use keyboards::{
    models::{genome, layout_map, letter_list},
    run_sa,
    setup::{COOLING_RATE, EPOCH, NUM_ITERATIONS, TEMPERATURE},
    SaveOption,
};

fn main() {
    let start_time = Utc::now().time();

    let result = run_sa(
        &layout_map::get_traditional_qwertz_layout_map(),
        &genome::QWERTZ_GENOME,
        &letter_list::LETTER_LIST_QWERTZ,
        TEMPERATURE,
        EPOCH,
        COOLING_RATE,
        NUM_ITERATIONS,
        // drawing the png files in Debug mode can be slow without optimizations
        SaveOption::Text,
    );

    let end_time = Utc::now().time();
    let diff = end_time - start_time;

    println!("Duration: {diff}");
    println!("Result: {:?}", result);
}

// #[cfg(test)]
// mod tests {
//     extern crate test;

//     use test::Bencher;

//     use super::*;

//     #[bench]
//     fn bench_add_two(b: &mut Bencher) {
//         b.iter(|| main());
//     }
// }
