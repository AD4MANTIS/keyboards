// ~~~ keyboard ~~~

use super::{Finger, Hand};

#[derive(Debug, Clone, Copy)]
pub enum KeyboardRow {
    Number = 0,
    TopLetter,
    MiddleLetter,
    BottomLetter,
}

#[derive(Debug)]
pub struct Layout {
    pub x: i32,
    pub y: i32,
    pub row: KeyboardRow,
    pub hand: Hand,
    pub finger: Finger,
    pub home: bool,
}

impl Layout {
    pub fn get_finger_id(&self) -> usize {
        (self.finger as usize)
            + match self.hand {
                Hand::Left => 0,
                Hand::Right => 4,
            }
    }
}

type RawLayout = (i32, i32, i32, i32, bool);

impl From<RawLayout> for Layout {
    fn from(value: RawLayout) -> Self {
        Self {
            x: value.0,
            y: value.1,
            row: match value.2 {
                1 => KeyboardRow::Number,
                2 => KeyboardRow::TopLetter,
                3 => KeyboardRow::MiddleLetter,
                4 => KeyboardRow::BottomLetter,
                _ => panic!(),
            },
            hand: match value.3 > 4 {
                true => Hand::Right,
                false => Hand::Left,
            },
            finger: match value.3 {
                1 | 8 => Finger::Ring,
                2 | 7 => Finger::Middle,
                3 | 6 => Finger::Index,
                4 | 5 => Finger::Thumb,
                _ => panic!(),
            },
            home: value.4,
        }
    }
}

// pub type LayoutMap = HashMap<usize, Layout>;

// fn into_layout_map(value: &[RawLayout]) -> LayoutMap {
//     value
//         .iter()
//         .map(|x| Layout::from(*x))
//         .into_iter()
//         .enumerate()
//         .collect()
// }

fn into_layout_map<const N: usize>(value: [RawLayout; N]) -> [Layout; N] {
    value
        .into_iter()
        .map(Layout::from)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[allow(dead_code)]
pub fn get_traditional_layout_map() -> [Layout; 46] {
    into_layout_map([
        (50, 450, 1, 1, false),
        (150, 450, 1, 1, false),
        (250, 450, 1, 1, false),
        (350, 450, 1, 2, false),
        (450, 450, 1, 3, false),
        (550, 450, 1, 4, false),
        (650, 450, 1, 4, false),
        (750, 450, 1, 5, false),
        (850, 450, 1, 6, false),
        (950, 450, 1, 7, false),
        (1050, 450, 1, 8, false),
        (1150, 450, 1, 8, false),
        (1250, 450, 1, 8, false),
        (200, 350, 2, 1, false),
        (300, 350, 2, 2, false),
        (400, 350, 2, 3, false),
        (500, 350, 2, 4, false),
        (600, 350, 2, 4, false),
        (700, 350, 2, 5, false),
        (800, 350, 2, 5, false),
        (900, 350, 2, 6, false),
        (1000, 350, 2, 7, false),
        (1100, 350, 2, 8, false),
        (1200, 350, 2, 8, false),
        (1300, 350, 2, 8, false),
        (225, 250, 3, 1, true),
        (325, 250, 3, 2, true),
        (425, 250, 3, 3, true),
        (525, 250, 3, 4, true),
        (625, 250, 3, 4, false),
        (725, 250, 3, 5, false),
        (825, 250, 3, 5, true),
        (925, 250, 3, 6, true),
        (1025, 250, 3, 7, true),
        (1125, 250, 3, 8, true),
        (1225, 250, 3, 8, false),
        (275, 150, 4, 1, false),
        (375, 150, 4, 2, false),
        (475, 150, 4, 3, false),
        (575, 150, 4, 4, false),
        (675, 150, 4, 4, false),
        (775, 150, 4, 5, false),
        (875, 150, 4, 5, false),
        (975, 150, 4, 6, false),
        (1075, 150, 4, 7, false),
        (1175, 150, 4, 8, false),
    ])
}

#[allow(dead_code)]
pub fn get_traditional_qwertz_layout_map() -> [Layout; 48] {
    into_layout_map([
        (50, 450, 1, 1, false),
        (150, 450, 1, 1, false),
        (250, 450, 1, 1, false),
        (350, 450, 1, 2, false),
        (450, 450, 1, 3, false),
        (550, 450, 1, 4, false),
        (650, 450, 1, 4, false),
        (750, 450, 1, 5, false),
        (850, 450, 1, 6, false),
        (950, 450, 1, 7, false),
        (1050, 450, 1, 8, false),
        (1150, 450, 1, 8, false),
        (1250, 450, 1, 8, false),
        (200, 350, 2, 1, false),
        (300, 350, 2, 2, false),
        (400, 350, 2, 3, false),
        (500, 350, 2, 4, false),
        (600, 350, 2, 4, false),
        (700, 350, 2, 5, false),
        (800, 350, 2, 5, false),
        (900, 350, 2, 6, false),
        (1000, 350, 2, 7, false),
        (1100, 350, 2, 8, false),
        (1200, 350, 2, 8, false),
        (1300, 350, 2, 8, false),
        (225, 250, 3, 1, true),
        (325, 250, 3, 2, true),
        (425, 250, 3, 3, true),
        (525, 250, 3, 4, true),
        (625, 250, 3, 4, false),
        (725, 250, 3, 5, false),
        (825, 250, 3, 5, true),
        (925, 250, 3, 6, true),
        (1025, 250, 3, 7, true),
        (1125, 250, 3, 8, true),
        (1225, 250, 3, 8, false),
        (1325, 250, 3, 8, false),
        (175, 150, 4, 1, false),
        (275, 150, 4, 1, false),
        (375, 150, 4, 2, false),
        (475, 150, 4, 3, false),
        (575, 150, 4, 4, false),
        (675, 150, 4, 4, false),
        (775, 150, 4, 5, false),
        (875, 150, 4, 5, false),
        (975, 150, 4, 6, false),
        (1075, 150, 4, 7, false),
        (1175, 150, 4, 8, false),
    ])
}
