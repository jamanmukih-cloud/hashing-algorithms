pub fn xxhash64(data: &[u8]) -> u64 {
    let mut h: u64 = 0x517cc1b727220a95;
    for byte in data {
        h = h.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(*byte as u64);
        h ^= h >> 31;
        h = h.wrapping_mul(0x9e3779b97f4a7c15);
        h ^= h >> 31;
    }
    h
}

pub struct ConsistentHash {
    ring: Vec<(u64, String)>,
}

impl ConsistentHash {
    pub fn new(nodes: Vec<&str>) -> Self {
        let mut ring: Vec<(u64, String)> = nodes.iter()
            .map(|n| (xxhash64(n.as_bytes()), n.to_string()))
            .collect();
        ring.sort_by_key(|&(k, _)| k);
        Self { ring }
    }
    
    pub fn get(&self, key: &str) -> &str {
        let h = xxhash64(key.as_bytes());
        for &(hash, ref node) in &self.ring {
            if hash >= h { return node; }
        }
        &self.ring[0].1
    }
}
