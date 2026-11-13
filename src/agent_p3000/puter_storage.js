// Puter.fs Sovereign Storage Wrapper for P3000_MPTIE
// Operates using puter.fs primitives for manifold state and artifact management

class PuterStorageEngine {
    constructor() {
        this.active = typeof puter !== 'undefined' && puter.fs;
    }

    async writeFile(path, data) {
        if (!this.active) throw new Error("Puter.fs context not initialized.");
        try {
            await puter.fs.write(path, data);
            console.log(`[PUTER.FS] Successfully wrote artifact to ${path}`);
        } catch (err) {
            console.error(`[PUTER.FS] Write failed for ${path}:`, err);
            throw err;
        }
    }

    async readFile(path) {
        if (!this.active) throw new Error("Puter.fs context not initialized.");
        try {
            const content = await puter.fs.read(path);
            return content;
        } catch (err) {
            console.error(`[PUTER.FS] Read failed for ${path}:`, err);
            throw err;
        }
    }

    async makeDirectory(path) {
        if (!this.active) throw new Error("Puter.fs context not initialized.");
        try {
            await puter.fs.mkdir(path, { recursive: true });
            console.log(`[PUTER.FS] Directory structured at ${path}`);
        } catch (err) {
            console.error(`[PUTER.FS] Mkdir failed for ${path}:`, err);
            throw err;
        }
    }

    async getStat(path) {
        if (!this.active) throw new Error("Puter.fs context not initialized.");
        return await puter.fs.stat(path);
    }

    async deleteItem(path) {
        if (!this.active) throw new Error("Puter.fs context not initialized.");
        await puter.fs.delete(path);
        console.log(`[PUTER.FS] Purged path: ${path}`);
    }

    async copyItem(source, destination) {
        if (!this.active) throw new Error("Puter.fs context not initialized.");
        await puter.fs.copy(source, destination);
        console.log(`[PUTER.FS] Cloned manifold asset from ${source} to ${destination}`);
    }

    async pasteItem(source, destinationDir) {
        if (!this.active) throw new Error("Puter.fs context not initialized.");
        await puter.fs.paste(source, destinationDir);
        console.log(`[PUTER.FS] Pasted asset ${source} into ${destinationDir}`);
    }
}

export const storageEngine = new PuterStorageEngine();
