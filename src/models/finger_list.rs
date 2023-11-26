// (homeX, homeY, currentX, currentY, distanceCounter, objectiveCounter)
pub type FingerList = [FingerListItem; 8];

pub fn get_finger_list() -> FingerList {
    [
        FingerListItem::default(),
        FingerListItem::default(),
        FingerListItem::default(),
        FingerListItem::default(),
        FingerListItem::default(),
        FingerListItem::default(),
        FingerListItem::default(),
        FingerListItem::default(),
    ]
}

#[derive(Debug, Clone, Default)]
pub struct FingerListItem {
    pub home_x: i32,
    pub home_y: i32,
    pub current_x: i32,
    pub current_y: i32,
    pub distance_counter: i32,
    pub objective_counter: f64,
}
