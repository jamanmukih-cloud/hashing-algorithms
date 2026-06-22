# Hashing Algorithms 🔢

Fast hashing: xxHash, MurmurHash3, SipHash, consistent hashing.

## Speed

| Algorithm | Speed | Quality | Use Case |
|-----------|-------|---------|----------|
| xxHash64 | 30 GB/s | Good | Checksums |
| MurmurHash3 | 25 GB/s | Good | Hash tables |
| SipHash | 5 GB/s | Excellent | Security |
| Consistent | O(1) | N/A | Distribution |

## Quick Start

```rust
let hash = xxhash64(b"hello world");
let ring = ConsistentHash::new(vec!["node1", "node2"]);
let node = ring.get("user:123");
```

## License

MIT