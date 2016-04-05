//! PeFile definitions.

use std::path::Path;
use std::fs::File;
use std::{io, mem, slice};
use std::io::{Seek, Read};

use super::peview::PeView;
use super::image::*;

//----------------------------------------------------------------

#[derive(Debug)]
pub enum PeError {
	/// There was an error reading the file.
	Io(io::Error),
	/// Magic values didn't match.
	BadMagic,
	/// Sanity checks failed.
	Insanity,
}

impl From<io::Error> for PeError {
	fn from(err: io::Error) -> PeError {
		PeError::Io(err)
	}
}

//----------------------------------------------------------------

/// Owned version of `super::peview::PeView`.
pub struct PeFile {
	buf: Vec<u8>,
}

impl PeFile {
	/// Read a PE file from disk.
	///
	/// # Parameters
	///
	/// * `path`
	///
	///   Path to the file on disk.
	///
	/// # Return value
	///
	/// `PeError::Io` error if any of the file reading fails.
	///
	/// `PeError::BadMagic` error if any of the PE magic values do not match.
	///
	/// `PeError::Insanity` error if any sanity check failed. If you get this error on a valid binary, you'll need to fix this library.
	///
	/// Otherwise the result is the correctly mapped binary.
	pub fn open(path: &Path) -> Result<PeFile, PeError> {
		let mut file = try!(File::open(path));
		let mut buf = Vec::<u8>::with_capacity(0x1000);

		//---------------- Read DOS header
		let dos_bytes = mem::size_of::<ImageDosHeader>();
		buf.resize(dos_bytes, 0);
		try!(file.read_exact(&mut buf[..]));

		//---------------- Get offset to NT headers
		let e_lfanew = {
			// This is safe since we read as many bytes earlier, `buf` shall not be modified in this block
			let dos = unsafe { &*(buf.as_ptr() as *const ImageDosHeader) };
			if dos.e_magic != IMAGE_DOS_HEADER_MAGIC {
				return Err(PeError::BadMagic);
			}
			// This is rather arbitrary as based on experience
			if dos.e_lfanew == 0 || dos.e_lfanew > 0x200 {
				return Err(PeError::Insanity);
			}
			dos.e_lfanew as usize
		};

		//---------------- Read up to and including NT headers
		let nt_bytes = e_lfanew + mem::size_of::<ImageNtHeaders>();
		buf.resize(nt_bytes, 0);
		try!(file.read_exact(&mut buf[dos_bytes..]));

		//---------------- Get NT headers information
		let (hdr_bytes, img_bytes, sec_begin, sec_num) = {
			// This is again safe, `buf` shall not be modified in this block
			let nt = unsafe { &*(buf.as_ptr().offset(e_lfanew as isize) as *const ImageNtHeaders) };

			if nt.Signature != IMAGE_NT_HEADERS_SIGNATURE || nt.OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR_MAGIC {
				return Err(PeError::BadMagic);
			}

			// These sanity checks are arbitrary as based on experience
			if nt.OptionalHeader.SizeOfHeaders > 0x1000 ||
				nt.OptionalHeader.NumberOfRvaAndSizes > IMAGE_NUMBEROF_DIRECTORY_ENTRIES as u32 ||
				nt.FileHeader.SizeOfOptionalHeader < mem::size_of::<ImageOptionalHeader>() as u16 ||
				nt.FileHeader.NumberOfSections > 100 {
				return Err(PeError::Insanity);
			}

			// Figure out section headers...
			let sec_begin = e_lfanew + (mem::size_of::<ImageNtHeaders>() - mem::size_of::<ImageOptionalHeader>()) + nt.FileHeader.SizeOfOptionalHeader as usize;
			let sec_end = sec_begin + nt.FileHeader.NumberOfSections as usize * mem::size_of::<ImageSectionHeader>();
			if sec_end > nt.OptionalHeader.SizeOfHeaders as usize {
				return Err(PeError::Insanity);
			}

			// (hdr_bytes, img_bytes, sec_begin, sec_num)
			(nt.OptionalHeader.SizeOfHeaders, nt.OptionalHeader.SizeOfImage, sec_begin, nt.FileHeader.NumberOfSections as usize)
		};

		//---------------- Allocate memory for entire image
		// Big realloc here, after this `buf` shall not be resized anymore
		buf.resize(img_bytes as usize, 0);

		//---------------- Read the section headers
		try!(file.read_exact(&mut buf[nt_bytes..hdr_bytes as usize]));
		// Invariant: during Self::map_sections no section shall overwrite the headers from under us making this safe
		let sections = unsafe { slice::from_raw_parts(buf.as_ptr().offset(sec_begin as isize) as *const ImageSectionHeader, sec_num) };

		//---------------- Map sections
		try!(Self::map_sections(&mut file, &mut buf[..], hdr_bytes, sections));

		//---------------- Done at last
		Ok(PeFile {
			buf: buf,
		})
	}
	fn map_sections(file: &mut File, buf: &mut [u8], min_rva: Rva, sections: &[ImageSectionHeader]) -> Result<(), PeError> {
		for it in sections {
			// Safety: `sections` is a slice of `buf` meaning we technically violate RwLock.
			//         This is safe however since `sections` is guaranteed to have an offset smaller than `min_rva`.
			if it.VirtualAddress < min_rva || it.VirtualSize == 0 {
				return Err(PeError::Insanity);
			}
			// Some sections are entirely zero initialized at runtime, they take no size on disk.
			if it.PointerToRawData != 0 {
				// Seek to the raw data pointer
				try!(file.seek(io::SeekFrom::Start(it.PointerToRawData as u64)));
				// FIXME! Validate these here so the next code can't panic!
				let begin = it.VirtualAddress as usize;
				let end = it.VirtualAddress as usize + it.SizeOfRawData as usize;
				// Read to the virtual address
				try!(file.read_exact(&mut buf[begin..end]));
			}
		}
		Ok(())
	}
	/// Get a view into the mapped image.
	pub fn view(&self) -> PeView {
		// With all the extensive error and sanity checking earlier, this better be safe...
		unsafe { PeView::module(self.buf.as_ptr()) }
	}
}
