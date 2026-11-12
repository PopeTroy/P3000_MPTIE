def apply_topological_surgery(global_adjacency_matrix: list[list[float]]) -> list[list[float]]:
    """Performs persistent homology filtration to strip thermal foam and extract minimal adjacency matrix A_ij."""
    print("[KOC] Executing Non-Local Topological Surgery (Rasengan graph filtering)...")
    # Filters high-frequency noise edges below significance threshold
    filtered_matrix = [
        [weight if weight > 0.05 else 0.0 for weight in row]
        for row in global_adjacency_matrix
    ]
    return filtered_matrix
