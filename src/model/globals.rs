pub struct Globals {
    pub bounds_min: Vec<f64>,
    pub bounds_max: Vec<f64>,
    pub radius: f64,
    pub n: u32,
}

impl Default for Globals {
    fn default() -> Self {
        Self {
            bounds_min: vec![-500.0, -250.0],
            bounds_max: vec![500.0, 250.0],
            radius: 75.0,
            n: 1024,
        }
    }
}
