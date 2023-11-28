use once_cell::sync::Lazy;

// rng
pub const SEED: u64 = 114211;

// weights
pub const DISTANCE_EFFORT: u32 = 1; // at 2 distance penalty is squared
pub const DOUBLE_FINGER_EFFORT: i32 = 1;
pub const DOUBLE_HAND_EFFORT: i32 = 1;

// typing speed
const FINGER_CPM: [i32; 8] = [223, 169, 225, 273, 343, 313, 259, 241];
static MEAN_CPM: Lazy<f64> =
    Lazy::new(|| FINGER_CPM.iter().sum::<i32>() as f64 / FINGER_CPM.len() as f64);
static STD_CPM: Lazy<f64> = Lazy::new(|| {
    (FINGER_CPM
        .iter()
        .map(|&x| (x as f64 - *MEAN_CPM).powi(2))
        .sum::<f64>()
        / FINGER_CPM.len() as f64)
        .sqrt()
});
static Z_SCORE_CPM: Lazy<Vec<f64>> = Lazy::new(|| {
    FINGER_CPM
        .iter()
        .map(|&x| -(x as f64 - *MEAN_CPM) / *STD_CPM)
        .collect()
});
pub(crate) static FINGER_EFFORT: Lazy<Vec<f64>> = Lazy::new(|| {
    Z_SCORE_CPM
        .iter()
        .map(|&x| x - Z_SCORE_CPM.iter().cloned().fold(f64::NAN, f64::min))
        .collect()
});

const ROW_CPM: [i32; 4] = [131, 166, 276, 192];
static MEAN_CPM2: Lazy<f64> =
    Lazy::new(|| ROW_CPM.iter().sum::<i32>() as f64 / ROW_CPM.len() as f64);
static STD_CPM2: Lazy<f64> = Lazy::new(|| {
    (ROW_CPM
        .iter()
        .map(|&x| (x as f64 - *MEAN_CPM2).powi(2))
        .sum::<f64>()
        / ROW_CPM.len() as f64)
        .sqrt()
});
static Z_SCORE_CPM2: Lazy<Vec<f64>> = Lazy::new(|| {
    ROW_CPM
        .iter()
        .map(|&x| -(x as f64 - *MEAN_CPM2) / *STD_CPM2)
        .collect()
});
pub(crate) static ROW_EFFORT: Lazy<Vec<f64>> = Lazy::new(|| {
    Z_SCORE_CPM2
        .iter()
        .map(|&x| x - Z_SCORE_CPM2.iter().cloned().fold(f64::NAN, f64::max))
        .collect()
});

pub(crate) const EFFORT_WEIGHTING: [f64; 5] = [0.7917, 1.0, 0.0, 0.4773, 0.0]; // dist, finger, row. Also had room for other weightings but removed for simplicity
