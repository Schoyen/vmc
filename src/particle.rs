#[derive(Debug, Clone)]
pub struct Particle {
    num_dimensions: usize,
    position: Vec<f64>,
}

impl Particle {
    pub fn new(num_dimensions: usize) -> Self {
        Particle {
            num_dimensions,
            position: vec![0.0; num_dimensions],
        }
    }

    pub fn set_position(&mut self, position: Vec<f64>) {
        self.position = position;
    }

    pub fn adjust_position(&mut self, step: f64, dim: usize) {
        self.position[dim] += step;
    }
}
