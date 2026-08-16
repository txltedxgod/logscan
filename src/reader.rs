use anyhow::{Context, Result};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

pub struct MmapLogReader {
    mmap: Mmap,
}

impl MmapLogReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(&path).with_context(|| format!("Failed to open file {:?}", path.as_ref()))?;
        let mmap = unsafe { Mmap::map(&file).context("Failed to memory-map file")? };
        Ok(Self { mmap })
    }

    pub fn size_bytes(&self) -> usize {
        self.mmap.len()
    }

    /// Splits memory-mapped bytes into chunks aligned to newline boundaries
    pub fn create_line_chunks(&self, num_chunks: usize) -> Vec<&[u8]> {
        let bytes = &self.mmap[..];
        let total_len = bytes.len();
        if total_len == 0 {
            return Vec::new();
        }

        let chunk_size = total_len / num_chunks.max(1);
        let mut chunks = Vec::new();
        let mut start = 0;

        while start < total_len {
            let mut end = (start + chunk_size).min(total_len);

            // If not at the end of the file, advance to next newline boundary
            if end < total_len {
                while end < total_len && bytes[end] != b'\n' {
                    end += 1;
                }
                if end < total_len && bytes[end] == b'\n' {
                    end += 1;
                }
            }

            chunks.push(&bytes[start..end]);
            start = end;
        }

        chunks
    }
}
