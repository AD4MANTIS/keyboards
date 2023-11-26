pub mod draw;
pub mod models;
pub mod setup;

use models::finger_list::FingerList;
use models::layout_map::Layout;
use models::Hand;
use once_cell::sync::Lazy;
use rand::prelude::*;
use rayon::prelude::*;
use setup::DISTANCE_EFFORT;
use setup::DOUBLE_FINGER_EFFORT;
use setup::DOUBLE_HAND_EFFORT;
use setup::KEY_MAP_DICT;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use crate::draw::draw_keyboard;
use crate::setup::BOOK_PATH;
use crate::setup::SEED;

const FINGER_CPM: [i32; 8] = [223, 169, 225, 273, 343, 313, 259, 241];
static MEAN_CPM: Lazy<f64> =
    Lazy::new(|| FINGER_CPM.iter().sum::<i32>() as f64 / FINGER_CPM.len() as f64);
static STD_CPM: Lazy<f64> = Lazy::new(|| {
    (FINGER_CPM
        .iter()
        .map(|&x| (x as f64 - *MEAN_CPM).powi(2))
        .sum::<f64>()
        / FINGER_CPM.len() as f64)
        .sqrt()
});
static Z_SCORE_CPM: Lazy<Vec<f64>> = Lazy::new(|| {
    FINGER_CPM
        .iter()
        .map(|&x| -(x as f64 - *MEAN_CPM) / *STD_CPM)
        .collect()
});
static FINGER_EFFORT: Lazy<Vec<f64>> = Lazy::new(|| {
    Z_SCORE_CPM
        .iter()
        .map(|&x| x - Z_SCORE_CPM.iter().cloned().fold(f64::NAN, f64::min))
        .collect()
});

const ROW_CPM: [i32; 4] = [131, 166, 276, 192];
static MEAN_CPM2: Lazy<f64> =
    Lazy::new(|| ROW_CPM.iter().sum::<i32>() as f64 / ROW_CPM.len() as f64);
static STD_CPM2: Lazy<f64> = Lazy::new(|| {
    (ROW_CPM
        .iter()
        .map(|&x| (x as f64 - *MEAN_CPM2).powi(2))
        .sum::<f64>()
        / ROW_CPM.len() as f64)
        .sqrt()
});
static Z_SCORE_CPM2: Lazy<Vec<f64>> = Lazy::new(|| {
    ROW_CPM
        .iter()
        .map(|&x| -(x as f64 - *MEAN_CPM2) / *STD_CPM2)
        .collect()
});
static ROW_EFFORT: Lazy<Vec<f64>> = Lazy::new(|| {
    Z_SCORE_CPM2
        .iter()
        .map(|&x| x - Z_SCORE_CPM2.iter().cloned().fold(f64::NAN, f64::max))
        .collect()
});

const EFFORT_WEIGHTING: [f64; 5] = [0.7917, 1.0, 0.0, 0.4773, 0.0]; // dist, finger, row. Also had room for other weightings but removed for simplicity

// ### KEYBOARD FUNCTIONS ###
fn create_genome<const N: usize>(letter_list: &mut [char; N], rng: &mut impl RngCore) -> [char; N] {
    letter_list.shuffle(rng);
    *letter_list
}

fn count_characters(book_path: &str) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    let file = File::open(book_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for c in line.unwrap().chars() {
            let c = c.to_ascii_uppercase();
            *char_count.entry(c).or_insert(0) += 1;
        }
    }

    char_count
}

// ### SAVE SCORE ###
fn append_to_file(path: &str, update_line: &str) {
    if !Path::new(path).exists() {
        File::create(path).unwrap_or_else(|_| panic!("Failed to create File {}", path));
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Failed to open file {}", path));

    writeln!(file, "{}", update_line)
        .unwrap_or_else(|_| panic!("Failed to write to file {}", path));
}

fn append_updates(update_line: &str) {
    append_to_file("results/iterationScores.txt", update_line);
}

// ### OBJECTIVE FUNCTIONS ###
fn determine_keypress(current_character: char) -> Option<usize> {
    // proceed if valid key (e.g. we don't care about spaces now)
    KEY_MAP_DICT.get(&current_character).map(|(kp, _)| *kp - 1)
}

fn do_keypress<const N: usize>(
    my_finger_list: &mut FingerList,
    my_genome: &[char; N],
    key_press: usize,
    old_finger: &mut i32,
    old_hand: &mut Option<Hand>,
    layout_map: &[Layout; N],
    letter_list: &[char; N],
) {
    let named_key = letter_list[key_press];
    let actual_key = my_genome.iter().position(|&x| x == named_key).unwrap();

    let Layout {
        x,
        y,
        row,
        hand,
        finger,
        home: _,
    } = layout_map[actual_key];
    let current_hand = hand;

    for finger_id in 0..8 {
        let (home_x, home_y, current_x, current_y, distance_counter, objective_counter) = (
            my_finger_list[finger_id][0],
            my_finger_list[finger_id][1],
            my_finger_list[finger_id][2],
            my_finger_list[finger_id][3],
            my_finger_list[finger_id][4],
            my_finger_list[finger_id][5],
        );

        if finger_id == finger as usize {
            let distance = (x - current_x).powi(2) + (y - current_y).powi(2);
            let distance_penalty = distance.powi(DISTANCE_EFFORT);
            let new_distance = distance_counter + distance;

            let double_finger_penalty =
                if finger as i32 != *old_finger && *old_finger != 0 && distance != 0. {
                    DOUBLE_FINGER_EFFORT
                } else {
                    0
                };
            *old_finger = finger as i32;

            let double_hand_penalty =
                if Some(current_hand) != *old_hand && *old_hand != Some(Hand::Left) {
                    DOUBLE_HAND_EFFORT
                } else {
                    0
                };
            *old_hand = Some(current_hand);

            let finger_penalty = FINGER_EFFORT[finger_id];
            let row_penalty = ROW_EFFORT[row as usize];

            let penalties = [
                distance_penalty,
                double_finger_penalty as f64,
                double_hand_penalty as f64,
                finger_penalty,
                row_penalty,
            ];
            let penalty = penalties
                .iter()
                .zip(EFFORT_WEIGHTING.iter())
                .map(|(x, y)| x * (*y))
                .sum::<f64>();
            let new_objective = objective_counter + penalty;

            my_finger_list[finger_id][2] = x;
            my_finger_list[finger_id][3] = y;
            my_finger_list[finger_id][4] = new_distance;
            my_finger_list[finger_id][5] = new_objective;
        } else {
            my_finger_list[finger_id][2] = home_x;
            my_finger_list[finger_id][3] = home_y;
        }
    }
}

/// Calculate the objective function for a given file, genome, and layout map
fn objective_function<const N: usize>(
    file: &str,
    my_genome: &[char; N],
    layout_map: &[Layout; N],
    layout_score: Option<f64>,
    letter_list: &[char; N],
) -> f64 {
    // create hand
    let mut my_finger_list: FingerList = [[0.0; 6]; 8]; // (homeX, homeY, currentX, currentY, distanceCounter, objectiveCounter)

    for &Layout {
        x,
        y,
        row: _,
        hand: _,
        finger,
        home,
    } in layout_map
    {
        if home {
            my_finger_list[finger as usize][0..4].copy_from_slice(&[x, y, x, y]);
        }
    }

    // load text
    let mut old_finger = 0;
    let mut old_hand: Option<Hand> = None;

    for current_character in file.chars() {
        // determine keypress
        let key_press = determine_keypress(current_character);

        // do keypress
        if let Some(kp) = key_press {
            do_keypress(
                &mut my_finger_list,
                my_genome,
                kp,
                &mut old_finger,
                &mut old_hand,
                layout_map,
                letter_list,
            );
        }
    }

    // calculate objective
    let mut objective = my_finger_list.iter().map(|finger| finger[5]).sum::<f64>();

    if let Some(layout_score) = layout_score {
        objective = (objective / layout_score - 1.0) * 100.0;
    }

    // return
    objective
}

/// Calculate the baseline objective function for a given file, genome, and layout map
fn baseline_objective_function<const N: usize>(
    file: &str,
    my_genome: &[char; N],
    layout_map: &[Layout; N],
    letter_list: &[char; N],
) -> f64 {
    objective_function(file, my_genome, layout_map, None, letter_list)
}

// ### SA OPTIMISER ###
fn shuffle_genome<const N: usize>(
    current_genome: &[char; N],
    temperature: f64,
    rng: &mut impl RngCore,
) -> [char; N] {
    // setup
    let no_switches = (temperature / 100.0).floor().min(N as f64).max(2.0) as usize;

    // positions of switched letterList
    let mut switched_positions: Vec<usize> = (0..N).collect();
    switched_positions.shuffle(rng);
    let switched_positions = &switched_positions[0..no_switches];

    let mut new_positions = switched_positions.to_vec();
    new_positions.shuffle(rng);

    // create new genome by shuffling
    let mut new_genome = *current_genome;
    for i in 0..no_switches {
        let og = switched_positions[i];
        let ne = new_positions[i];
        new_genome[og] = current_genome[ne];
    }

    new_genome
}

pub enum SaveOption {
    None,
    Graphics,
    Text,
}

pub fn run_sa<const N: usize>(
    layout_map: &[Layout; N],
    baseline_layout: &[char; N],
    letter_list: &[char; N],
    mut temperature: f64,
    epoch: f64,
    cooling_rate: f64,
    num_iterations: usize,
    save_current_best: SaveOption,
) -> [char; N] {
    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

    let mut letter_list = *letter_list;
    let mut file = File::open(BOOK_PATH).expect("Unable to open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read file");

    println!("Calculating raw baseline: ");
    // baseline
    print!("Calculating raw baseline: ");

    let layout_score =
        baseline_objective_function(&file_content, baseline_layout, layout_map, &letter_list);
    println!("{}", layout_score);

    println!(
        "From here everything is reletive with + % worse and - % better than this baseline \n
        Note that best layout is being saved as a png at each step. Kill program when satisfied."
    );

    println!("Temperature \t Best Score \t New Score");

    let mut current_genome = create_genome(&mut letter_list, &mut rng);
    let mut current_objective = objective_function(
        &file_content,
        &current_genome,
        layout_map,
        Some(layout_score),
        &letter_list,
    );

    let mut best_genome = current_genome;
    let mut best_objective = current_objective;

    draw_keyboard(&best_genome, "0", layout_map);
    append_updates("\nStarting new Run");
    append_updates("temperature | iteration | bestObjective | newObjective");

    // run SA
    let mut static_count = 0.0;
    let mut iteration = 0;
    while iteration <= num_iterations && temperature > 1.0 {
        iteration += 1;
        // ~ create new genome ~
        let new_genome = shuffle_genome(&current_genome, 2., &mut rng);

        // ~ asess ~
        let new_objective = objective_function(
            &file_content,
            &new_genome,
            layout_map,
            Some(layout_score),
            &letter_list,
        );
        let delta = new_objective - current_objective;

        println!("{temperature:.2}\t{best_objective:.2}\t{new_objective:.2}");

        if delta < 0.0 {
            current_genome = new_genome;
            current_objective = new_objective;

            let update_line =
                format!("{temperature:.2}, {iteration}, {best_objective:.5}, {new_objective:.5}");
            append_updates(&update_line);

            if new_objective < best_objective {
                best_genome = new_genome;
                best_objective = new_objective;

                println!("(new best, png being saved)");
                match save_current_best {
                    SaveOption::Graphics => {
                        draw_keyboard(&best_genome, iteration.to_string().as_str(), layout_map);
                    }
                    SaveOption::Text => {
                        append_to_file(
                            "resources/bestGenomes.txt",
                            &format!("{iteration}: {best_genome:#?}\n"),
                        );
                    }
                    SaveOption::None => {}
                }
            }
        } else if (-delta / temperature).exp() > rand::random() {
            current_genome = new_genome;
            current_objective = new_objective;
        }

        static_count += 1.0;

        if static_count > epoch {
            static_count = 0.0;
            temperature *= cooling_rate;

            if rand::random::<f64>() < 0.5 {
                current_genome = best_genome;
                current_objective = best_objective;
            }
        }
    }

    draw_keyboard(&best_genome, "final", layout_map);

    best_genome
}
