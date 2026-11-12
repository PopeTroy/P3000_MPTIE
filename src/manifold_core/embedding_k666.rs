use nalgebra::DVector;

/// Fractal Projection Operator Phi_k mapping high-dim observations to M_k
pub fn projection_kernel_k666(h_t: &DVector<f64>, centroids: &[DVector<f64>], k_coeff: f64) -> DVector<f64> {
    let lambda = 1.0;
    let mut embedded = DVector::zeros(h_t.nrows());

    for (i, c_i) in centroids.iter().enumerate() {
        let r = (h_t - c_i).norm();
        // Mittag-Leffler Kernel approximation for heavy-tailed temporal dependence
        let weight = r.powf(-k_coeff) * (-r / lambda).exp();
        embedded += h_t * weight;
    }
    embedded
}
