// #![feature(test)]

use chrono::Utc;
use keyboards::prelude::*;

fn main() {
    let start_time = Utc::now().time();

    let training_set_path = "resources/meinBuch.txt";

    let file_content = std::fs::read_to_string(training_set_path).expect("Unable to open file");

    // Select your layout with the first generic parameter.
    // The second ist the number of keys and must match the Layout, otherwise you will get a compiler error.
    let result = run_sa::<QwertyEnUs, 46>(
        &file_content,
        SaSetup {
            temperature: 500.,
            epoch: 20.,
            cooling_rate: 0.99,
            num_iterations: 25000,
        },
        LoggingOptions {
            text: true,
            image: SaveImageOption::FirstAndLast,
            verbosity: Verbosity::Normal,
        },
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
//     fn bench_run_sa(b: &mut Bencher) {
//         b.iter(|| main());
//     }
// }
