#[derive(Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub density: f64,
}

impl From<Vec<f64>> for Particle {
    fn from(position: Vec<f64>) -> Self {
        let velocity = position.iter().map(|_| 0.0).collect();
        Self {
            position: position,
            velocity: velocity,
            density: 0.0,
        }
    }
}
