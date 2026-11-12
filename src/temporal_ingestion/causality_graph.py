class CausalityDAG:
    def __init__(self):
        self.nodes = set()
        self.edges = []

    def add_causal_edge(self, source: str, target: str, transfer_entropy: float):
        self.nodes.add(source)
        self.nodes.add(target)
        self.edges.append((source, target, transfer_entropy))

    def compute_adjacency_matrix(self) -> list[list[float]]:
        node_list = sorted(list(self.nodes))
        size = len(node_list)
        matrix = [[0.0]*size for _ in range(size)]
        
        node_to_idx = {node: i for i, node in enumerate(node_list)}
        for src, tgt, weight in self.edges:
            i = node_to_idx[src]
            j = node_to_idx[tgt]
            matrix[i][j] = weight
        return matrix
