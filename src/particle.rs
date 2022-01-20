use rand::{prelude::thread_rng, Rng};

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

    pub fn compute_pos_squared(&self) -> f64 {
        self.position.iter().map(|x| x * x).sum()
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

    pub fn get_particle_pos_copy(&self, p_index: usize) -> Vec<f64> {
        self.particles[p_index].position.to_vec()
    }

    pub fn get_particle_pos(&self, p_index: usize) -> &Vec<f64> {
        &self.particles[p_index].position
    }

    pub fn get_distance_between_particles(
        &self,
        p_i: usize,
        p_j: usize,
    ) -> f64 {
        let pos_i = &self.particles[p_i].position;
        let pos_j = &self.particles[p_j].position;

        pos_i
            .iter()
            .zip(pos_j.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum()
    }

    pub fn set_particle_pos(&mut self, p_index: usize, pos: Vec<f64>) {
        self.particles[p_index].set_position(pos);
    }

    pub fn get_num_dimensions(&self) -> usize {
        self.num_dimensions
    }

    pub fn get_num_particles(&self) -> usize {
        self.particles.len()
    }

    pub fn distribute_particles(&mut self, spread: f64) {
        let mut rng = thread_rng();

        // TODO: Make sure particles aren't too close...
        for particle in &mut self.particles {
            let mut position = vec![0.0; self.num_dimensions];
            for i in 0..self.num_dimensions {
                position[i] += spread * rng.gen_range(-1.0..1.0);
            }

            particle.set_position(position);
        }
    }

    pub fn compute_pos_squared_sum(&self) -> f64 {
        self.particles.iter().map(|x| x.compute_pos_squared()).sum()
    }

    pub fn move_particle(&mut self, step: f64, p_index: usize, dim: usize) {
        self.particles[p_index].adjust_position(step, dim);
    }
}
