use super::Wavefunction;
use crate::particle::Particles;

pub struct InteractingEllipticGaussian {
    parameters: Vec<f64>,
    beta: f64,
    a: f64,
}

impl InteractingEllipticGaussian {
    pub fn new(alpha: f64, beta: f64, a: f64) -> Self {
        InteractingEllipticGaussian {
            parameters: vec![alpha],
            beta,
            a,
        }
    }

    fn compute_pos_squared(&self, pos: &Vec<f64>) -> f64 {
        let mut pos_sq_sum = 0.0;

        for i in 0..pos.len() - 1 {
            pos_sq_sum += pos[i].powi(2);
        }

        pos_sq_sum += (pos[pos.len() - 1] * self.beta).powi(2);

        pos_sq_sum
    }

    fn evaluate_single_particle_function(
        &self,
        p_i: usize,
        particles: &Particles,
    ) -> f64 {
        let alpha = self.parameters[0];
        let position = particles.get_particle_pos(p_i);

        (-alpha * self.compute_pos_squared(position)).exp()
    }

    fn evaluate_correlation_wavefunction(
        &self,
        p_i: usize,
        p_j: usize,
        particles: &Particles,
    ) -> f64 {
        let dist: f64 = particles.get_distance_between_particles(p_i, p_j);

        if dist <= self.a {
            return 0.0;
        }

        1.0 - self.a / dist
    }

    fn compute_laplacian_of_single_particle(
        &self,
        p_i: usize,
        particles: &Particles,
    ) -> f64 {
        let mut laplacian = self.compute_spf_laplacian(p_i, particles);
        let gradient_spf = self.compute_spf_gradient(p_i, particles);
        let gradient_cw = self.compute_cw_gradient(p_i, particles);

        for k in 0..particles.get_num_dimensions() {
            laplacian += 2.0 * gradient_spf[k] * gradient_cw[k];
            laplacian += gradient_cw[k].powi(2);
        }

        laplacian += self.compute_cw_laplacian(p_i, particles);

        laplacian
    }

    fn compute_spf_laplacian(&self, p_i: usize, particles: &Particles) -> f64 {
        let alpha = self.parameters[0];
        let pos_i = particles.get_particle_pos(p_i);
        let pos_sq_sum = self.compute_pos_squared(pos_i);
        let num_dims = pos_i.len() as f64;

        let mut laplacian = -2.0 * alpha * (num_dims - 1.0 + self.beta);
        laplacian += 4.0 * alpha.powi(2) * pos_sq_sum;

        laplacian
    }

    fn compute_spf_gradient(
        &self,
        p_i: usize,
        particles: &Particles,
    ) -> Vec<f64> {
        let alpha = self.parameters[0];
        let pos_i = particles.get_particle_pos(p_i);
        let mut grad = vec![0.0; pos_i.len()];

        for i in 0..(pos_i.len() - 1) {
            grad[i] += -2.0 * alpha * pos_i[i];
        }

        grad[pos_i.len() - 1] +=
            -2.0 * alpha * self.beta * pos_i[pos_i.len() - 1];

        grad
    }

    fn compute_cw_gradient(
        &self,
        p_i: usize,
        particles: &Particles,
    ) -> Vec<f64> {
        let pos_i = particles.get_particle_pos(p_i);

        let mut gradient = vec![0.0; pos_i.len()];

        for p_j in 0..particles.get_num_particles() {
            if p_j == p_i {
                continue;
            }

            let pos_j = particles.get_particle_pos(p_j);
            let dist = particles.get_distance_between_particles(p_i, p_j);

            for i in 0..pos_i.len() {
                gradient[i] += (pos_i[i] - pos_j[i]) * self.a
                    / (dist.powi(2) * (dist - self.a));
            }
        }

        gradient
    }

    fn compute_cw_laplacian(&self, p_i: usize, particles: &Particles) -> f64 {
        let mut laplacian = 0.0;

        for p_j in 0..particles.get_num_particles() {
            if p_i == p_j {
                continue;
            }

            let dist = particles.get_distance_between_particles(p_i, p_j);

            laplacian += ((particles.get_num_dimensions() as f64) - 1.0)
                * self.a
                / (dist.powi(2) * (dist - self.a))
                + (dist.powi(2) - 2.0 * self.a * dist)
                    / (dist.powi(2) * (dist - self.a).powi(2));
        }

        laplacian
    }
}

impl Wavefunction for InteractingEllipticGaussian {
    fn get_num_parameters(&self) -> usize {
        self.parameters.len()
    }

    fn get_parameters(&self) -> &Vec<f64> {
        &self.parameters
    }

    fn evaluate(&self, particles: &Particles) -> f64 {
        let mut product = 1.0;

        for p_i in 0..particles.get_num_particles() {
            product *= self.evaluate_single_particle_function(p_i, particles);
            for p_j in (p_i + 1)..particles.get_num_particles() {
                product *=
                    self.evaluate_correlation_wavefunction(p_i, p_j, particles);
            }
        }

        product
    }

    fn compute_laplacian(&self, particles: &Particles) -> f64 {
        let mut laplacian = 0.0;

        for p_i in 0..particles.get_num_particles() {
            laplacian +=
                self.compute_laplacian_of_single_particle(p_i, particles);
        }

        laplacian
    }

    fn compute_gradient_of_particle(
        &self,
        p_i: usize,
        particles: &Particles,
    ) -> Vec<f64> {
        let gradient_spf = self.compute_spf_gradient(p_i, particles);
        let gradient_cw = self.compute_cw_gradient(p_i, particles);

        gradient_spf
            .iter()
            .zip(gradient_cw.iter())
            .map(|(x, y)| x + y)
            .collect()
    }
}
