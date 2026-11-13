// Puter.fs Sovereign File Operations Wrapper
class PuterSovereignFS {
    static async initializeManifoldDirectory() {
        await puter.fs.mkdir("/p3000_manifolds", { recursive: true });
        console.log("[PUTER.FS] Sovereign directory structured: /p3000_manifolds");
    }

    static async persistState(slotId, statePacket) {
        const path = `/p3000_manifolds/worker_${slotId}_state.json`;
        await puter.fs.write(path, JSON.stringify(statePacket, null, 2));
        console.log(`[PUTER.FS] State checkpoint written to ${path}`);
    }

    static async inspectState(slotId) {
        const path = `/p3000_manifolds/worker_${slotId}_state.json`;
        const stats = await puter.fs.stat(path);
        const content = await puter.fs.read(path);
        return { stats, content: JSON.parse(content) };
    }

    static async cloneCheckpoint(sourceId, targetId) {
        const src = `/p3000_manifolds/worker_${sourceId}_state.json`;
        const dest = `/p3000_manifolds/worker_${targetId}_state.json`;
        await puter.fs.copy(src, dest);
        console.log(`[PUTER.FS] Cloned checkpoint from slot ${sourceId} to ${targetId}`);
    }

    static async purgeCheckpoint(slotId) {
        const path = `/p3000_manifolds/worker_${slotId}_state.json`;
        await puter.fs.delete(path);
        console.log(`[PUTER.FS] Purged checkpoint for slot ${slotId}`);
    }
}
