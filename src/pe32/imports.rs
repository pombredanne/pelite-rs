//! PE imports.

use std::{fmt, mem};

use super::image::*;
use super::peview::PeView;

//----------------------------------------------------------------

/// Imported symbol.
pub enum ImportedSymbol<'a> {
	/// Imported by name.
	///
	/// The hint is an ordinal in the export table that may contain the desired symbol.
	/// For more information see: https://blogs.msdn.microsoft.com/oldnewthing/20100317-00/?p=14573
	ByName { hint: u16, name: &'a str },
	/// Imported by ordinal.
	ByOrdinal { ord: u16 }
}

impl<'a> fmt::Display for ImportedSymbol<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ImportedSymbol::ByName { hint: _, name } => {
				write!(f, "{}", name)
			},
			ImportedSymbol::ByOrdinal { ord } => {
				write!(f, "#{}", ord)
			},
		}
	}
}

//----------------------------------------------------------------

/// Imports directory.
pub struct ImportDirectory<'a: 'b, 'b> {
	view_: &'b PeView<'a>,
	datadir_: &'a ImageDataDirectory,
}

impl<'a, 'b> ImportDirectory<'a, 'b> {
	/// Get the associated `PeView`.
	#[inline]
	pub fn view(&self) -> &PeView {
		self.view_
	}
	/// Iterate over the import descriptors.
	#[inline]
	pub fn iter(&'a self) -> ImportDescriptorIterator<'a, 'b> {
		ImportDescriptorIterator {
			view: self.view_,
			it: self.datadir_.VirtualAddress,
		}
	}
}

impl<'a, 'b> fmt::Display for ImportDirectory<'a, 'b> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for desc in self.iter() {
			try!(write!(f, "{}", desc));
		}
		Ok(())
	}
}

//----------------------------------------------------------------

pub trait PeImports {
	fn imports(&self) -> Option<ImportDirectory>;
}

impl<'a> PeImports for PeView<'a> {
	fn imports(&self) -> Option<ImportDirectory> {
		if let Some(datadir) = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_IMPORT) {
			if datadir.VirtualAddress != BADRVA {
				Some(ImportDirectory {
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

pub struct ImportDescriptorIterator<'a: 'b, 'b> {
	view: &'b PeView<'a>,
	it: Rva,
}

impl<'a, 'b> Iterator for ImportDescriptorIterator<'a, 'b> {
	type Item = ImportDescriptor<'a, 'b>;

	fn next(&mut self) -> Option<Self::Item> {
		let image = self.view.read_struct::<ImageImportDescriptor>(self.it).unwrap();
		fn is_sentinel(image: &ImageImportDescriptor) -> bool {
			// Documentation says all fields must be zeroed,
			// but you can (probably) get away just checking OriginalFirstThunk...
			image.OriginalFirstThunk == BADRVA &&
			image.TimeDateStamp == BADRVA &&
			image.ForwarderChain == BADRVA &&
			image.Name == BADRVA &&
			image.FirstThunk == BADRVA
		}
		if is_sentinel(image) {
			None
		}
		else {
			self.it += mem::size_of::<ImageImportDescriptor>() as Rva;
			Some(ImportDescriptor {
				view_: self.view,
				image_: image,
			})
		}
	}
}

//----------------------------------------------------------------

pub struct ImportDescriptor<'a: 'b, 'b> {
	view_: &'b PeView<'a>,
	image_: &'a ImageImportDescriptor,
}

impl<'a, 'b> ImportDescriptor<'a, 'b> {
	/// Get the associated `PeView`.
	#[inline]
	pub fn view(&self) -> &'b PeView {
		self.view_
	}
	/// Get the underlying import descriptor image.
	#[inline]
	pub fn image(&self) -> &'a ImageImportDescriptor {
		self.image_
	}
	/// Get the DLL name imported from.
	#[inline]
	pub fn dll_name(&self) -> &'a str {
		self.view_.read_str(self.image_.Name).unwrap()
	}
	/// Iterate over the import name table.
	#[inline]
	pub fn int_iter(&self) -> ImportNameIterator {
		ImportNameIterator {
			desc: self,
			it: self.image_.OriginalFirstThunk,
		}
	}
	/// Iterate over the import address table.
	#[inline]
	pub fn iat_iter(&self) -> ImportTableIterator {
		ImportTableIterator {
			desc: self,
			it: self.image_.FirstThunk,
		}
	}
}

impl<'a, 'b> fmt::Display for ImportDescriptor<'a, 'b> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "Imports from {}", self.dll_name()));
		try!(writeln!(f, "  TimeDateStamp:  {}", self.image_.TimeDateStamp));
		try!(writeln!(f, "  ForwarderChain: {:>08X}", self.image_.ForwarderChain));
		try!(writeln!(f, "  IAT:            {:>08X}", self.image_.FirstThunk));
		for thunk in self.int_iter() {
			try!(writeln!(f, "  {}", thunk));
		}
		Ok(())
	}
}

//----------------------------------------------------------------

pub struct ImportNameIterator<'a: 'b, 'b> {
	desc: &'b ImportDescriptor<'a, 'b>,
	it: Rva,
}

impl<'a, 'b> Iterator for ImportNameIterator<'a, 'b> {
	type Item = ImportedSymbol<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let va = self.desc.view_.read_struct::<Va>(self.it).unwrap();
		if *va != BADVA {
			self.it += mem::size_of::<Va>() as Rva;
			if *va & IMAGE_ORDINAL_FLAG == 0 {
				let hint = self.desc.view_.read_struct::<u16>(*va as Rva).unwrap();
				let name = self.desc.view_.read_str(*va as Rva + 2).unwrap();
				Some(ImportedSymbol::ByName { hint: *hint, name: name })
			}
			else {
				Some(ImportedSymbol::ByOrdinal { ord: (*va & 0xFFFF) as u16 })
			}
		}
		else {
			None
		}
	}
}

pub struct ImportTableIterator<'a: 'b, 'b> {
	desc: &'b ImportDescriptor<'a, 'b>,
	it: Rva,
}

impl<'a, 'b> Iterator for ImportTableIterator<'a, 'b> {
	type Item = &'a Va;

	fn next(&mut self) -> Option<Self::Item> {
		let va = self.desc.view_.read_struct::<Va>(self.it).unwrap();
		if *va != BADVA {
			self.it += mem::size_of::<Va>() as Rva;
			Some(va)
		}
		else {
			None
		}
	}
}
