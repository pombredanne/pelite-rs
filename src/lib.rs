// Reexported later under pe32 and pe64.
mod image;

pub mod pe32;
pub mod pe64;
pub mod resources;

/// Defaults to the current platform if it is available.
#[cfg(all(windows, target_pointer_width = "32"))]
pub use pe32 as pe;
/// Defaults to the current platform if it is available.
#[cfg(all(windows, target_pointer_width = "64"))]
pub use pe64 as pe;
