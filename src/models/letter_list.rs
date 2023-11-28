use super::layout::{QwertyEnUs, QwertzDeDe};

// alphabet
pub trait GetLetterList<const N: usize> {
    fn get_letter_list() -> &'static [char; N];
}

impl GetLetterList<46> for QwertyEnUs {
    fn get_letter_list() -> &'static [char; 46] {
        &LETTER_LIST_QWERTY
    }
}

impl GetLetterList<48> for QwertzDeDe {
    fn get_letter_list() -> &'static [char; 48] {
        &LETTER_LIST_QWERTZ
    }
}

const LETTER_LIST_QWERTY: [char; 46] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '~', '-',
    '+', '[', ']', ';', '\'', '<', '>', '?',
];

const LETTER_LIST_QWERTZ: [char; 48] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '^', 'ß',
    '´', 'Ü', 'Ö', 'Ä', '+', '#', '<', ',', '.', '-',
];
