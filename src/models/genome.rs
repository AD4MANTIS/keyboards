use rand::{seq::SliceRandom, RngCore};

use super::layout::{QwertyEnUs, QwertzDeDe};

pub(crate) fn create_genome<const N: usize>(
    letter_list: &mut [char; N],
    rng: &mut impl RngCore,
) -> [char; N] {
    letter_list.shuffle(rng);
    *letter_list
}

// SA OPTIMIZER
pub(crate) fn shuffle_genome<const N: usize>(
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

// comparisons
// initial index defines the starting key a character will be placed on
pub trait GetGenome<const N: usize> {
    fn get_genome() -> &'static [char; N];
}

impl GetGenome<46> for QwertyEnUs {
    fn get_genome() -> &'static [char; 46] {
        &QWERTY_GENOME
    }
}

impl GetGenome<48> for QwertzDeDe {
    fn get_genome() -> &'static [char; 48] {
        &QWERTZ_GENOME
    }
}

#[allow(dead_code)]
pub const QWERTY_GENOME: [char; 46] = [
    '~', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '+', 'Q', 'W', 'E', 'R', 'T', 'Y',
    'U', 'I', 'O', 'P', '[', ']', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', '\'', 'Z', 'X',
    'C', 'V', 'B', 'N', 'M', '<', '>', '?',
];

#[allow(dead_code)]
pub const QWERTZ_GENOME: [char; 48] = [
    '^', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'ß', '´', 'Q', 'W', 'E', 'R', 'T', 'Z',
    'U', 'I', 'O', 'P', 'Ü', '+', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Ö', 'Ä', '#', '<',
    'Y', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '-',
];

#[allow(dead_code)]
pub const ABC_GENOME: [char; 46] = [
    '~', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '+', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'I', 'J', '[', ']', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', ';', '\'', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '<', '>', '?',
];

#[allow(dead_code)]
pub const DVORAK_GENOME: [char; 46] = [
    '~', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '[', ']', '\'', '<', '>', 'P', 'Y', 'F',
    'G', 'C', 'R', 'L', '?', '+', 'A', 'O', 'E', 'U', 'I', 'D', 'H', 'T', 'N', 'S', '-', ';', 'Q',
    'J', 'K', 'X', 'B', 'M', 'W', 'V', 'Z',
];
