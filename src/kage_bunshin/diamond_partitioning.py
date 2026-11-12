class CausalDiamondPartition:
    def __init__(self, total_epochs: int = 3000, num_clones: int = 5):
        self.total_epochs = total_epochs
        self.num_clones = num_clones

    def partition_timeline(self) -> list[tuple[int, int]]:
        span = self.total_epochs // self.num_clones
        diamonds = []
        for i in range(self.num_clones]:
            start = (i * span) + 1
            end = self.total_epochs if i == self.num_clones - 1 else (i + 1) * span
            diamonds.append((start, end))
        return diamonds

if __name__ == "__main__":
    partitioner = CausalDiamondPartition()
    print("[KBC] Causal Diamonds partitioned:", partitioner.partition_timeline())
  
