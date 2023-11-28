pub(crate) mod finger_list;
pub(crate) mod genome;
pub(crate) mod key_maps;
pub mod layout;
pub(crate) mod layout_map;
pub(crate) mod letter_list;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Finger {
    Ring = 0,
    Middle,
    Index,
    Thumb,
}
