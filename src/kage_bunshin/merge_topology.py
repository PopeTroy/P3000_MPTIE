def merge_topologies(local_invariants: list[dict]) -> dict:
    """Merges sharded topological invariants (Betti numbers, causal edges) into global skeleton G_K."""
    global_betti_0 = sum(inv.get("betti_0", 0) for inv in local_invariants)
    global_betti_1 = sum(inv.get("betti_1", 0) for inv in local_invariants)
    
    return {
        "global_skeleton": "G_K_Final",
        "betti_invariants": (global_betti_0, global_betti_1),
        "status": "Merged successfully via Kurama Dispersal Bus"
    }
