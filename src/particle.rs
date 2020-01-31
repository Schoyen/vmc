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

#[derive(Debug)]
pub struct Particles {
    num_dimensions: usize,
    particles: Vec<Particle>,
}

impl Particles {
    pub fn new(num_particles: usize, num_dimensions: usize) -> Self {
        Particles {
            num_dimensions,
            particles: vec![Particle::new(num_dimensions); num_particles],
        }
    }

    pub fn get_num_dimensions(&self) -> usize {
        self.num_dimensions
    }

    pub fn get_num_particles(&self) -> usize {
        self.particles.len()
    }
}
