use std::collections::BTreeMap;

use crate::advanced_setup::{
    DISTANCE_EFFORT, DOUBLE_FINGER_EFFORT, DOUBLE_HAND_EFFORT, EFFORT_WEIGHTING, FINGER_EFFORT,
    ROW_EFFORT,
};
use crate::models::finger_list::{get_finger_list, FingerList};
use crate::models::layout::ILayout;
use crate::models::layout_map::KeyboardKey;
use crate::models::Hand;

// OBJECTIVE FUNCTIONS
pub(crate) fn determine_keypress<L: ILayout<N>, const N: usize>(
    current_character: char,
) -> Option<usize> {
    // proceed if valid key (e.g. we don't care about spaces now)
    L::get_key_map(current_character).map(|(kp, _)| kp - 1)
}

pub(crate) fn do_keypress<const N: usize>(
    my_finger_list: &mut FingerList,
    my_genome: &BTreeMap<char, usize>,
    key_press: usize,
    old_finger: &mut usize,
    old_hand: &mut Option<Hand>,
    layout_map: &[KeyboardKey; N],
    letter_list: &[char; N],
) {
    let named_key = letter_list[key_press];
    let actual_key = my_genome.get(&named_key).expect("Key should be in genome");

    let layout = &layout_map[*actual_key];
    let current_hand = layout.hand;
    let layout_finger_id = layout.get_finger_id();

    for (finger_id, my_finger) in my_finger_list.iter_mut().enumerate() {
        if finger_id == layout_finger_id {
            continue;
        }

        my_finger.current_y = my_finger.home_y;
        my_finger.current_x = my_finger.home_x;
    }

    let finger_id = layout_finger_id;
    let my_finger = &mut my_finger_list[finger_id];

    let distance =
        (layout.x.abs_diff(my_finger.current_x) + layout.y.abs_diff(my_finger.current_y)) as i32;
    let distance_penalty = distance.pow(DISTANCE_EFFORT);
    let new_distance = my_finger.distance_counter + distance;

    let double_finger_penalty = if finger_id != *old_finger && *old_finger != 0 && distance != 0 {
        DOUBLE_FINGER_EFFORT
    } else {
        0
    };

    *old_finger = finger_id;

    let double_hand_penalty = if Some(current_hand) != *old_hand && *old_hand != Some(Hand::Left) {
        DOUBLE_HAND_EFFORT
    } else {
        0
    };
    *old_hand = Some(current_hand);

    let finger_penalty = FINGER_EFFORT[finger_id];
    let row_penalty = ROW_EFFORT[layout.row as usize];

    let penalties = [
        distance_penalty as f64,
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
    let new_objective = my_finger.objective_counter + penalty;

    my_finger.current_x = layout.x;
    my_finger.current_y = layout.y;
    my_finger.distance_counter = new_distance;
    my_finger.objective_counter = new_objective;
}

/// Calculate the objective function for a given file, genome, and layout map
pub(crate) fn objective_function<L: ILayout<N>, const N: usize>(
    key_presses: &[usize],
    my_genome: &[char; N],
    layout_map: &[KeyboardKey; N],
    layout_score: Option<f64>,
    letter_list: &[char; N],
) -> f64 {
    // create hand
    let mut my_finger_list: FingerList = get_finger_list();

    for layout in layout_map {
        if layout.home {
            let my_finger = &mut my_finger_list[layout.get_finger_id()];
            my_finger.home_x = layout.x;
            my_finger.home_y = layout.y;
            my_finger.current_x = layout.x;
            my_finger.current_y = layout.y;
        }
    }

    // load text
    let mut old_finger: usize = 0;
    let mut old_hand: Option<Hand> = None;

    let genome_key_map =
        BTreeMap::<char, usize>::from_iter(my_genome.iter().enumerate().map(|x| (*x.1, x.0)));

    for key_press in key_presses {
        do_keypress(
            &mut my_finger_list,
            &genome_key_map,
            *key_press,
            &mut old_finger,
            &mut old_hand,
            layout_map,
            letter_list,
        );
    }

    // calculate objective
    let mut objective = my_finger_list
        .iter()
        .map(|finger| finger.objective_counter)
        .sum::<f64>();

    if let Some(layout_score) = layout_score {
        objective = (objective / layout_score - 1.0) * 100.0;
    }

    // return
    objective
}

/// Calculate the baseline objective function for a given file, genome, and layout map
pub(crate) fn baseline_objective_function<L: ILayout<N>, const N: usize>(
    key_presses: &[usize],
    my_genome: &[char; N],
    layout_map: &[KeyboardKey; N],
    letter_list: &[char; N],
) -> f64 {
    objective_function::<L, N>(key_presses, my_genome, layout_map, None, letter_list)
}
