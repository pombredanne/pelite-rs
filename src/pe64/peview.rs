//! PeView definitions.

use std::{mem, slice, str};

use super::image::*;

/// PeView provides interaction with a mapped PE image.
///
/// PE images on disk have a different representation than those mapped to memory.
/// In memory each section is aligned to page size (typically 4K), on disk this is a waste of space and uses a different alignment.
/// Make sure to map the image to memory before using it with `PeView`!
pub struct PeView<'a> {
	image: &'a [u8],
	vbase: Va,
}

impl<'a> PeView<'a> {
	/// Create a new instance of PeView of the module this code is executing in.
	#[cfg(all(windows, target_pointer_width = "64"))]
	pub fn new() -> PeView<'a> {
		// Should be safe, unless you go fucking around with stuff like erasing PE headers. Don't do that.
		unsafe { Self::module(image_base() as *const _ as *const u8) }
	}
	/// Create a new instance of PeView of a mapped module.
	///
	/// # Parameters
	///
	/// * `base`
	///
	///   Pointer to the mapped module in memory.
	///
	/// # Return value
	///
	/// View into memory pointed at by `base` interpreted as a PE module.
	///
	/// # Safety
	///
	/// The underlying memory is not taken ownership of. Make sure it outlives this PeView instance!
	///
	/// No sanity or safety checks are done to make sure this is really a PE64 module.
	/// When using this with a `HMODULE` from the system the caller must be sure this is a PE64 module, ie this is a 64 bit process.
	pub unsafe fn module(base: *const u8) -> PeView<'a> {
		let dos = &*(base as *const ImageDosHeader);
		let nt = &*(base.offset(dos.e_lfanew as isize) as *const ImageNtHeaders);
		PeView {
			image: slice::from_raw_parts(base, nt.OptionalHeader.SizeOfImage as usize),
			vbase: nt.OptionalHeader.ImageBase,
		}
	}
	/// Get the mapped image as a byte slice.
	pub fn image(&self) -> &[u8] {
		self.image
	}
	/// Get the virtual base address.
	pub fn virtual_base(&self) -> Va {
		self.vbase
	}
	/// Get the dos header image.
	pub fn dos_header(&self) -> &ImageDosHeader {
		unsafe {
			// Checked in validate() so this is safe
			&*(self.image.as_ptr() as *const ImageDosHeader)
		}
	}
	/// Get the NT headers image.
	pub fn nt_headers(&self) -> &ImageNtHeaders {
		let dos = self.dos_header();
		// Checked in validate() so this is safe
		unsafe { &*((dos as *const _ as *const u8).offset(dos.e_lfanew as isize) as *const ImageNtHeaders) }
	}
	/// Get the file header image.
	pub fn file_header(&self) -> &ImageFileHeader {
		&self.nt_headers().FileHeader
	}
	/// Get the optional header image.
	pub fn optional_header(&self) -> &ImageOptionalHeader {
		&self.nt_headers().OptionalHeader
	}
	/// Get the section image headers.
	pub fn section_headers(&self) -> &[ImageSectionHeader] {
		let nt = self.nt_headers();
		// Checked in validate() so this is safe
		unsafe {
			let begin = (&nt.OptionalHeader as *const _ as *const u8).offset(nt.FileHeader.SizeOfOptionalHeader as isize) as *const ImageSectionHeader;
			slice::from_raw_parts(begin, nt.FileHeader.NumberOfSections as usize)
		}
	}
	/// Get the data directory.
	pub fn data_directory(&self) -> &[ImageDataDirectory] {
		let opt = self.optional_header();
		// Checked in validate() so this is safe
		unsafe { slice::from_raw_parts(opt.DataDirectory.as_ptr(), opt.NumberOfRvaAndSizes as usize) }
	}
	/// Interpret as struct.
	///
	/// # Parameters
	///
	/// * `T`
	///
	///   Type of the struct to cast as.
	///   This should be a POD type without references or fancy shenanigans!
	///
	/// * `rva`
	///
	///   Rva pointing to the instance to interpret as `T`.
	///
	/// # Return value
	///
	/// If `rva` is `BADRVA` the result is `None`.
	/// No data is copied, a pointer to the underlying bytes is casted to a `&T`.
	///
	/// # Panics
	///
	/// If `rva` is out of range or has the wrong alignment.
	///
	/// This typically means data somewhere was corrupted resulting in an invalid `rva`.
	/// Corruption may trigger a panic but it is not guaranteed if the result happens to look correct.
	/// At no point will it read out of bounds memory.
	pub fn read_struct<T>(&self, rva: Rva) -> Option<&T> {
		if rva == BADRVA {
			None
		}
		else {
			let rva = rva as usize;
			assert!(rva <= self.image.len() - mem::size_of::<T>()); // Note! This assert will pass on underflow...
			assert!(rva % mem::align_of::<T>() == 0);
			// This is now safe
			let ptr = unsafe { self.image.as_ptr().offset(rva as isize) };
			Some(unsafe { &*(ptr as *const T) })
		}
	}
	/// Interpret as slice.
	///
	/// # Parameters
	///
	/// * `T`
	///
	///   Type of the slice.
	///   This should be a POD type without references or fancy shenanigans!
	///
	/// * `rva`
	///
	///   Rva pointing to an array of `T` to be interpreted as a slice.
	///
	/// * `len`
	///
	///   Number of elements in the array pointed at by `rva`.
	///
	/// # Return value
	///
	/// If `rva` is `BADRVA` the result is `None`.
	/// No data is copied, a pointer to the underlying bytes is casted to a `&[T]` with length `len`.
	///
	/// # Panics
	///
	/// If `rva` is out of range or has the wrong alignment.
	///
	/// This typically means data somewhere was corrupted resulting in an invalid `rva`.
	/// Corruption may trigger a panic but it is not guaranteed if the result happens to look correct.
	/// At no point will it read out of bounds memory.
	pub fn read_slice<T>(&self, rva: Rva, len: usize) -> Option<&[T]> {
		if rva == BADRVA {
			None
		}
		else {
			let rva = rva as usize;
			assert!(rva <= self.image.len() - mem::size_of::<T>() * len); // Note! this assert will pass on underflow...
			assert!(rva % mem::align_of::<T>() == 0);
			// This is now safe
			let ptr = unsafe { self.image.as_ptr().offset(rva as isize) };
			Some(unsafe { slice::from_raw_parts(ptr as *const T, len) })
		}
	}
	/// Interpret as str.
	///
	/// # Parameters
	///
	/// * `rva`
	///
	///   Rva pointing to a valid UTF8, null terminated C string.
	///
	/// # Return value
	///
	/// If `rva` is `BADRVA` the result is `None`.
	/// No data is copied, a pointer to the underlying bytes is casted to a `&str`.
	///
	/// # Panics
	///
	/// If `rva` is out of range or points to invalid UTF8.
	///
	/// This typically means data somewhere was corrupted resulting in an invalid `rva`.
	/// Corruption may trigger a panic but it is not guaranteed if the result happens to look correct.
	/// At no point will it read out of bounds memory.
	pub fn read_str(&self, rva: Rva) -> Option<&str> {
		if rva == BADRVA {
			None
		}
		else {
			let rva = rva as usize;
			// Scan for the null byte
			for i in 0usize.. {
				if self.image[rva + i] == 0u8 {
					// Found length, create a slice out of it
					let str = unsafe { slice::from_raw_parts(self.image.as_ptr().offset(rva as isize), i) };
					// Convert to str
					return Some(str::from_utf8(str).unwrap());
				}
			}
			unreachable!();
		}
	}
	/// Convert an Rva to FileOffset.
	///
	/// # Parameters
	///
	/// * `rva`
	///
	///   Rva to convert.
	///
	/// # Return value
	///
	/// `None` for invalid `rva`. Else the FileOffset to this `rva`.
	pub fn rva_to_file_offset(&self, rva: Rva) -> Option<FileOffset> {
		for it in self.section_headers() {
			if rva >= it.VirtualAddress && rva < (it.VirtualAddress + it.SizeOfRawData) {
				return Some((rva - it.VirtualAddress + it.PointerToRawData) as FileOffset);
			}
		}
		None
	}
	/// Convert a FileOffset to Rva.
	///
	/// # Parameters
	///
	/// * `file_offset`
	///
	///   FileOffset to convert.
	///
	/// # Return value
	///
	/// `BADRVA` for invalid `file_offset`. Else the Rva to this `file_offset`.
	pub fn file_offset_to_rva(&self, file_offset: FileOffset) -> Rva {
		for it in self.section_headers() {
			if file_offset >= it.PointerToRawData as FileOffset && file_offset < (it.PointerToRawData as FileOffset + it.SizeOfRawData as FileOffset) {
				return file_offset as Rva - it.PointerToRawData + it.VirtualAddress;
			}
		}
		BADRVA
	}
	/// Convert an Rva to Va.
	///
	/// # Parameters
	///
	/// * `rva`
	///
	///   Rva to convert.
	///
	/// # Return value
	///
	/// `BADVA` if `rva` is `BADRVA`.
	///
	/// # Remarks
	///
	/// The `rva` parameter isn't sanity checked to make sure it points within this image.
	pub fn rva_to_va(&self, rva: Rva) -> Va {
		if rva != BADRVA { self.vbase + rva as Va }
		else { BADVA }
	}
	/// Convert a Va to Rva.
	///
	/// # Parameters
	///
	/// * `va`
	///
	///   Va to convert.
	///
	/// # Return value
	///
	/// `BADRVA` if `va` is `BADVA`.
	///
	/// # Remarks
	///
	/// The `va` parameter isn't sanity checked to make sure it points within this image.
	///
	/// **FIXME!** This is especially problematic in PE64 images where Va is 64 bit and the resulting value doesn't fit in a 32 bit Rva...
	pub fn va_to_rva(&self, va: Va) -> Rva {
		if va != BADVA {
			// FIXME! Overflow or underflow are very unsafe here!
			(va - self.vbase) as Rva
		}
		else {
			BADRVA
		}
	}
}
