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
    pub x: f64,
    pub y: f64,
    pub row: KeyboardRow,
    pub hand: Hand,
    pub finger: Finger,
    pub home: bool,
}

type RawLayout = (f64, f64, i32, i32, bool);

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
        (0.5, 4.5, 1, 1, false),
        (1.5, 4.5, 1, 1, false),
        (2.5, 4.5, 1, 1, false),
        (3.5, 4.5, 1, 2, false),
        (4.5, 4.5, 1, 3, false),
        (5.5, 4.5, 1, 4, false),
        (6.5, 4.5, 1, 4, false),
        (7.5, 4.5, 1, 5, false),
        (8.5, 4.5, 1, 6, false),
        (9.5, 4.5, 1, 7, false),
        (10.5, 4.5, 1, 8, false),
        (11.5, 4.5, 1, 8, false),
        (12.5, 4.5, 1, 8, false),
        (2., 3.5, 2, 1, false),
        (3., 3.5, 2, 2, false),
        (4., 3.5, 2, 3, false),
        (5., 3.5, 2, 4, false),
        (6., 3.5, 2, 4, false),
        (7., 3.5, 2, 5, false),
        (8., 3.5, 2, 5, false),
        (9., 3.5, 2, 6, false),
        (10., 3.5, 2, 7, false),
        (11., 3.5, 2, 8, false),
        (12., 3.5, 2, 8, false),
        (13., 3.5, 2, 8, false),
        (2.25, 2.5, 3, 1, true),
        (3.25, 2.5, 3, 2, true),
        (4.25, 2.5, 3, 3, true),
        (5.25, 2.5, 3, 4, true),
        (6.25, 2.5, 3, 4, false),
        (7.25, 2.5, 3, 5, false),
        (8.25, 2.5, 3, 5, true),
        (9.25, 2.5, 3, 6, true),
        (10.25, 2.5, 3, 7, true),
        (11.25, 2.5, 3, 8, true),
        (12.25, 2.5, 3, 8, false),
        (2.75, 1.5, 4, 1, false),
        (3.75, 1.5, 4, 2, false),
        (4.75, 1.5, 4, 3, false),
        (5.75, 1.5, 4, 4, false),
        (6.75, 1.5, 4, 4, false),
        (7.75, 1.5, 4, 5, false),
        (8.75, 1.5, 4, 5, false),
        (9.75, 1.5, 4, 6, false),
        (10.75, 1.5, 4, 7, false),
        (11.75, 1.5, 4, 8, false),
    ])
}

#[allow(dead_code)]
pub fn get_traditional_qwertz_layout_map() -> [Layout; 48] {
    into_layout_map([
        (0.5, 4.5, 1, 1, false),
        (1.5, 4.5, 1, 1, false),
        (2.5, 4.5, 1, 1, false),
        (3.5, 4.5, 1, 2, false),
        (4.5, 4.5, 1, 3, false),
        (5.5, 4.5, 1, 4, false),
        (6.5, 4.5, 1, 4, false),
        (7.5, 4.5, 1, 5, false),
        (8.5, 4.5, 1, 6, false),
        (9.5, 4.5, 1, 7, false),
        (10.5, 4.5, 1, 8, false),
        (11.5, 4.5, 1, 8, false),
        (12.5, 4.5, 1, 8, false),
        (2., 3.5, 2, 1, false),
        (3., 3.5, 2, 2, false),
        (4., 3.5, 2, 3, false),
        (5., 3.5, 2, 4, false),
        (6., 3.5, 2, 4, false),
        (7., 3.5, 2, 5, false),
        (8., 3.5, 2, 5, false),
        (9., 3.5, 2, 6, false),
        (10., 3.5, 2, 7, false),
        (11., 3.5, 2, 8, false),
        (12., 3.5, 2, 8, false),
        (13., 3.5, 2, 8, false),
        (2.25, 2.5, 3, 1, true),
        (3.25, 2.5, 3, 2, true),
        (4.25, 2.5, 3, 3, true),
        (5.25, 2.5, 3, 4, true),
        (6.25, 2.5, 3, 4, false),
        (7.25, 2.5, 3, 5, false),
        (8.25, 2.5, 3, 5, true),
        (9.25, 2.5, 3, 6, true),
        (10.25, 2.5, 3, 7, true),
        (11.25, 2.5, 3, 8, true),
        (12.25, 2.5, 3, 8, false),
        (13.25, 2.5, 3, 8, false),
        (1.75, 1.5, 4, 1, false),
        (2.75, 1.5, 4, 1, false),
        (3.75, 1.5, 4, 2, false),
        (4.75, 1.5, 4, 3, false),
        (5.75, 1.5, 4, 4, false),
        (6.75, 1.5, 4, 4, false),
        (7.75, 1.5, 4, 5, false),
        (8.75, 1.5, 4, 5, false),
        (9.75, 1.5, 4, 6, false),
        (10.75, 1.5, 4, 7, false),
        (11.75, 1.5, 4, 8, false),
    ])
}
