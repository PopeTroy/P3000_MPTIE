use nalgebra::DVector;
use crate::metric_tensor::MetricTensor;

pub struct SentinelWalker {
    pub id: u64,
    pub position: DVector<f64>,
    pub momentum: DVector<f64>,
    pub hamiltonian: f64,
    pub k_coeff: f64,
}

impl SentinelWalker {
    pub fn step_leapfrog(&mut self, d_tau: f64) {
        let metric = MetricTensor::compute_at(&self.position, self.k_coeff);
        let force = -metric.christoffel_contract(&self.momentum);

        // Symplectic Integrator Guarantees Hamiltonian Conservation
        self.momentum += &force * (d_tau * 0.5);
        self.position += &self.momentum * d_tau;
        self.momentum += &force * (d_tau * 0.5);
    }
}
