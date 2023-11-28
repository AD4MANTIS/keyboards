use super::{
    genome::GetGenome, key_maps::GetKeyMap, layout_map::GetLayoutMap, letter_list::GetLetterList,
};

#[derive(Debug, Clone, Copy)]
pub enum Layout {
    QwertyEnUs,
    QwertzDeDe,
}

pub trait ILayout<const N: usize>:
    GetGenome<N> + GetKeyMap + GetLetterList<N> + GetLayoutMap<N>
{
    fn get() -> Layout;
}

pub struct QwertyEnUs();

impl ILayout<46> for QwertyEnUs {
    fn get() -> Layout {
        Layout::QwertyEnUs
    }
}

pub struct QwertzDeDe();

impl ILayout<48> for QwertzDeDe {
    fn get() -> Layout {
        Layout::QwertzDeDe
    }
}
