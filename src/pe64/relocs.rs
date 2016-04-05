//! PE relocations.
//!
//! For a quick overview how relocs work, see this excellent [stackoverflow answer](https://stackoverflow.com/a/22513813).

use std::{mem, fmt};

use super::image::*;
use super::peview::PeView;

//----------------------------------------------------------------

/// Relocations directory.
pub struct RelocsDirectory<'a> {
	view_: &'a PeView<'a>,
	datadir_: &'a ImageDataDirectory,
}

impl<'a> RelocsDirectory<'a> {
	/// Get the associated `PeView`.
	pub fn view(&self) -> &PeView {
		self.view_
	}
	/// Iterate over the relocations.
	pub fn iter(&self) -> RelocsIterator {
		RelocsIterator {
			relocs: self,
			it: self.datadir_.VirtualAddress,
		}
	}
}

impl<'a> fmt::Display for RelocsDirectory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for it in self.iter() {
			try!(write!(f, "{}", it));
		}
		Ok(())
	}
}

//----------------------------------------------------------------

pub trait PeRelocs {
	fn relocs(&self) -> Option<RelocsDirectory>;
}

impl<'a> PeRelocs for PeView<'a> {
	fn relocs(&self) -> Option<RelocsDirectory> {
		if let Some(datadir) = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_BASERELOC) {
			if datadir.VirtualAddress != BADRVA {
				Some(RelocsDirectory {
					view_: self,
					datadir_: datadir,
				})
			}
			else {
				None
			}
		}
		else {
			None
		}
	}
}

//----------------------------------------------------------------

pub struct RelocsIterator<'a> {
	relocs: &'a RelocsDirectory<'a>,
	it: Rva,
}

impl<'a> Iterator for RelocsIterator<'a> {
	type Item = BaseRelocations<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let end = self.relocs.datadir_.VirtualAddress + self.relocs.datadir_.Size;
		if self.it >= end {
			None
		}
		else {
			// Get the base relocation
			let rel = self.relocs.view_.read_struct::<ImageBaseRelocation>(self.it).unwrap();
			// Sanity check, without this underflow later can be very unsafe
			assert!(rel.SizeOfBlock as usize > mem::size_of::<ImageBaseRelocation>());
			// Get the number of base reloc blocks
			let block_len = (rel.SizeOfBlock as usize - mem::size_of::<ImageBaseRelocation>()) / mem::size_of::<ImageBaseRelocBlock>();
			// Get the blocks as a slice
			let blocks = self.relocs.view_.read_slice::<ImageBaseRelocBlock>(self.it + mem::size_of::<ImageBaseRelocation>() as Rva, block_len).unwrap();
			// Advance iterator
			self.it += rel.SizeOfBlock;
			Some(BaseRelocations {
				view_: self.relocs.view_,
				reloc_: rel,
				blocks_: blocks,
			})
		}
	}
}

//----------------------------------------------------------------

pub struct BaseRelocations<'a> {
	view_: &'a PeView<'a>,
	reloc_: &'a ImageBaseRelocation,
	blocks_: &'a [ImageBaseRelocBlock],
}

impl<'a> BaseRelocations<'a> {
	/// Get the associated `PeView`.
	pub fn view(&self) -> &PeView {
		self.view_
	}
	/// Get the base relocation image.
	pub fn image(&self) -> &ImageBaseRelocation {
		self.reloc_
	}
	/// Get the base reloc blocks as a slice.
	pub fn blocks(&self) -> &[ImageBaseRelocBlock] {
		self.blocks_
	}
	/// Get the final Rva of a reloc block.
	pub fn rva_of(&self, block: &ImageBaseRelocBlock) -> Rva {
		let offset = (block.TypeAndOffset & 0x0FFF) as Rva;
		self.reloc_.VirtualAddress + offset
	}
	/// Get the type of a reloc block.
	pub fn type_of(&self, block: &ImageBaseRelocBlock) -> u8 {
		((block.TypeAndOffset >> 12) & 0xFF) as u8
	}
}

impl<'a> fmt::Display for BaseRelocations<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "BaseRelocations"));
		try!(writeln!(f, "  VirtualAddress: {:>08X}", self.reloc_.VirtualAddress));
		try!(writeln!(f, "  SizeOfBlock:    {:>08X}", self.reloc_.SizeOfBlock));
		for it in self.blocks() {
			try!(writeln!(f, "  Type: {:>01X} Offset: {:>03X}", it.TypeAndOffset >> 12, it.TypeAndOffset & 0x0FFF));
		}
		Ok(())
	}
}
