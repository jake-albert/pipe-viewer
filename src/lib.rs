pub mod args;
pub mod read;
pub mod stats;
pub mod write;

// 16 KiB to begin with. Adjust later if needed.
const CHUNK_SIZE: usize = 16 * 1024;
