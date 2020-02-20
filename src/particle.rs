use rand::{
    prelude::{random, thread_rng},
    Rng,
};

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

    pub fn get_particle_pos(&self, p_index: usize) -> Vec<f64> {
        // TODO: Check if p_index can be given a fixed size
        self.particles[p_index].position.to_vec()
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
            let position =
                vec![spread * rng.gen_range(-1.0, 1.0); self.num_dimensions];
            particle.set_position(position);
        }
    }

    pub fn compute_pos_squared_sum(&self) -> f64 {
        let mut squared_sum = 0.0;

        for particle in &self.particles {
            squared_sum += particle.compute_pos_squared();
        }

        squared_sum
    }

    pub fn propose_move(
        &mut self,
        p_index: usize,
        step_length: f64,
        all_dims: bool,
    ) {
        let mut rng = thread_rng();

        let particle = &mut self.particles[p_index];

        if all_dims {
            for dim in 0..self.num_dimensions {
                let step = step_length * rng.gen_range(-1.0, 1.0);
                particle.adjust_position(step, dim);
            }
        } else {
            let dim = random::<usize>() % self.num_dimensions;
            let step = step_length * rng.gen_range(-1.0, 1.0);
            particle.adjust_position(step, dim);
        }
    }
}
