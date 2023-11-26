// comparisons
// initial index defines the starting key a character will be placed on

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
