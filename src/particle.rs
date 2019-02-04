#[derive(Debug)]
pub struct Particle {
    position: Vec<f64>,
}

impl Particle {
    pub fn new(position: Vec<f64>) -> Self {
        Particle { position }
    }

    pub fn move_particle(&mut self, step: f64, dim: u8) {
        self.position[dim as usize] += step;
    }
}
