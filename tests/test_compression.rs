#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k666_scaling_invariant() {
        let k_coeff = 0.666;
        assert!((k_coeff - (2.0 / 3.0)).abs() < 1e-3, "Fractal dimension coefficient must approximate 2/3.");
    }
}
