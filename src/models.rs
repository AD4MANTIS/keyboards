pub mod finger_list;
pub mod genome;
pub mod key_maps;
pub mod layout_map;
pub mod letter_list;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Finger {
    Ring = 0,
    Middle,
    Index,
    Thumb,
}
