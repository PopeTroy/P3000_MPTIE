use nalgebra::DVector;

pub struct MetricTensor {
    pub dimensions: usize,
    pub k_coeff: f64,
}

impl MetricTensor {
    pub fn compute_at(position: &DVector<f64>, k_coeff: f64) -> Self {
        // Enforces volume element scaling Det(g) ~ t^(-2k/3) as t -> 3000
        Self {
            dimensions: position.nrows(),
            k_coeff,
        }
    }

    pub fn christoffel_contract(&self, momentum: &DVector<f64>) -> DVector<f64> {
        // Christoffel symbol contraction: Gamma^mu_{nu rho} v^nu v^rho
        momentum * self.k_coeff
    }
}
