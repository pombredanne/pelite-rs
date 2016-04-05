//! PE structures.

pub use super::super::image::*;

pub const IMAGE_NT_OPTIONAL_HDR_MAGIC: u16 = IMAGE_NT_OPTIONAL_HDR64_MAGIC;

pub type ImageOptionalHeader = ImageOptionalHeader64;
pub type ImageNtHeaders = ImageNtHeaders64;

pub const IMAGE_ORDINAL_FLAG: u64 = IMAGE_ORDINAL_FLAG64;

/// Relative virtual address type, these are all offsets from the base of the mapped image in memory.
pub type Rva = u32;
/// Virtual address type, absolute address as known by the image. Not always the same as a pointer.
pub type Va = u64;
/// FileOffset type, when dealing with file offsets.
pub type FileOffset = usize;

/// Invalid Rva value.
pub const BADRVA: Rva = 0;
/// Invalid Va value.
pub const BADVA: Va = 0;
