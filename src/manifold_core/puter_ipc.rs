use std::fs::File;
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifoldStatePacket {
    pub dimension_coefficient: f64,
    pub bento_betti_signature: u32,
    pub entropy_foam_level: f64,
}

pub struct PuterFSIpc;

impl PuterFSIpc {
    pub fn puter_fs_write(path: &str, packet: &ManifoldStatePacket) -> io::Result<()> {
        let serialized = serde_json::to_string(packet)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        println!("[PUTER.FS RUST] State successfully persisted via sovereign IPC to {}", path);
        Ok(())
    }

    pub fn puter_fs_read(path: &str) -> io::Result<ManifoldStatePacket> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let packet: ManifoldStatePacket = serde_json::from_str(&contents)?;
        Ok(packet)
    }
}
