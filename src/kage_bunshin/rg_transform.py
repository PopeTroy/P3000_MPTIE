import math

def apply_local_rg_transform(state_tensor: list[float], scale_factor: float = 0.666) -> list[float]:
    """Applies local Renormalization Group transform with fractal dimension k=0.666."""
    coarse_state = [val * math.pow(scale_factor, 1.5) for val in state_tensor]
    return coarse_state
  
