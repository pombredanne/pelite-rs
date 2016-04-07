//! PE exports.

use std::{fmt};

use super::image::*;
use super::peview::PeView;

//----------------------------------------------------------------

/// Exported symbol.
pub enum Export<'a> {
	/// Symbol does not exist.
	///
	/// This can happen when exports are manually defined by ordinal and there are gaps,
	/// in this case not every entry in the exports has a valid symbol.
	None,
	/// Standard exported symbol.
	Symbol(&'a Rva),
	/// This export is forwarded to another dll.
	///
	/// Format of the string is `DllName.ExportName`.
	/// For more information see: https://blogs.msdn.microsoft.com/oldnewthing/20060719-24/?p=30473
	Forward(&'a str),
}

impl<'a> fmt::Display for Export<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Export::None => {
				write!(f, "None")
			},
			Export::Symbol(rva) => {
				write!(f, "{:>08X}", rva)
			},
			Export::Forward(str) => {
				write!(f, "{}", str)
			},
		}
	}
}

/// Full symbol information, including its ordinal and name (if it has any).
///
/// It's mainly used for pretty printing exports as it's not very efficient if you don't need the name.
pub struct NamedExport<'a> {
	/// Ordinal of this symbol.
	pub ord: u16,
	/// Resolved symbol value.
	pub symbol: Export<'a>,
	/// Name by which this symbol was exported.
	///
	/// `None` means the symbol has no name and was exported by ordinal if `symbol` is not `Export::None`.
	pub name: Option<&'a str>,
}

impl<'a> fmt::Display for NamedExport<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.symbol {
			Export::None => {
				write!(f, "None")
			},
			Export::Symbol(&rva) => {
				if let Some(name) = self.name {
					write!(f, "{:>08X} {}", rva, name)
				}
				else {
					write!(f, "{:>08X} #{}", rva, self.ord)
				}
			},
			Export::Forward(str) => {
				if let Some(name) = self.name {
					write!(f, "FORWARD {} to {}", name, str)
				}
				else {
					write!(f, "FORWARD #{} to {}", self.ord, str)
				}
			},
		}
	}
}

//----------------------------------------------------------------

/// Exports directory.
pub struct ExportDirectory<'a: 'b, 'b> {
	view_: &'b PeView<'a>,
	datadir_: &'a ImageDataDirectory,
	image_: &'a ImageExportDirectory,
}

impl<'a, 'b> ExportDirectory<'a, 'b> {
	/// Get the associated `PeView`.
	#[inline]
	pub fn view(&self) -> &'b PeView<'a> {
		self.view_
	}
	/// Get the underlying export directory image.
	#[inline]
	pub fn image(&self) -> &'a ImageExportDirectory {
		self.image_
	}
	/// Get the export directory's name for this library.
	#[inline]
	pub fn name(&self) -> &'a str {
		self.view_.read_str(self.image_.Name).unwrap()
	}
	/// Get the export address table.  
	#[inline]
	pub fn functions(&self) -> Option<&'a [Rva]> {
		self.view_.read_slice(self.image_.AddressOfFunctions, self.image_.NumberOfFunctions as usize)
	}
	/// Get the name address table.  
	#[inline]
	pub fn names(&self) -> Option<&'a [Rva]> {
		self.view_.read_slice(self.image_.AddressOfNames, self.image_.NumberOfNames as usize)
	}
	/// Get the name ordinal index table.
	///
	/// The value in this array is an index (not an ordinal!) into the export address table matching name in the same index as the name address table.
	#[inline]
	pub fn name_indices(&self) -> Option<&'a [u16]> {
		self.view_.read_slice(self.image_.AddressOfNameOrdinals, self.image_.NumberOfNames as usize)
	}
	/// If this is a forwarded export.
	///
	/// # Parameters
	///
	/// * `rva`
	///
	///   Rva of the symbol being checked.
	///
	/// # Return value
	///
	/// Returns if `rva` is a forwarded symbol.
	#[inline]
	pub fn is_forwarded(&self, rva: Rva) -> bool {
		rva >= self.datadir_.VirtualAddress && rva < self.datadir_.VirtualAddress + self.datadir_.Size
	}
	/// Find a symbol by its ordinal.
	///
	/// # Parameters
	///
	/// * `ord`
	///
	///   Ordinal of the symbol to find.
	///
	/// # Return value
	///
	/// `Export` value.
	pub fn symbol_by_ordinal(&self, ord: u16) -> Export<'a> {
		if let Some(functions) = self.functions() {
			let ord_idx = ord - self.image_.Base as u16;
			if let Some(sym_rva) = functions.get(ord_idx as usize) {
				if *sym_rva != BADRVA {
					return self.symbol_from_rva(sym_rva);
				}
			}
		}
		Export::None
	}
	/// Find a symbol by its name.
	///
	/// # Parameters
	///
	/// * `name`
	///
	///   Name of the symbol to find.
	///
	/// # Return value
	///
	/// `Export` value.
	pub fn symbol_by_name(&self, name: &str) -> Export<'a> {
		if let Some(functions) = self.functions() {
		if let Some(names) = self.names() {
		if let Some(name_indices) = self.names() {
			for (&name_rva, &name_ord_idx) in names.iter().zip(name_indices.iter()) {
				let name_it = self.view_.read_str(name_rva).unwrap();
				if name_it == name {
					if let Some(sym_rva) = functions.get(name_ord_idx as usize) {
						if *sym_rva != BADRVA {
							return self.symbol_from_rva(sym_rva);
						}
					}
					// Export table is corrupt, shouldn't happen...
					return Export::None;
				}
			}
		}}}
		Export::None
	}
	/// Find the name for an export.
	///
	/// # Parameters
	///
	/// * `ord`
	///
	///   Ordinal of the symbol to find.
	///
	/// # Return value
	///
	/// `NamedExport` value.
	pub fn name_from_ordinal(&self, ord: u16) -> NamedExport<'a> {
		if let Some(functions) = self.functions() {
			let ord_idx = ord - self.image_.Base as u16;
			if let Some(sym_rva) = functions.get(ord_idx as usize) {
				if *sym_rva != BADRVA {
					if let Some(name_indices) = self.name_indices() {
					if let Some(names) = self.names() {
						for (&name_rva, &name_ord_idx) in names.iter().zip(name_indices.iter()) {
							if ord_idx == name_ord_idx {
								return NamedExport {
									ord: ord,
									symbol: self.symbol_from_rva(sym_rva),
									name: Some(self.view_.read_str(name_rva).unwrap()),
								};
							}
						}
					}}
					return NamedExport {
						ord: ord,
						symbol: self.symbol_from_rva(sym_rva),
						name: None,
					};
				}
			}
		}
		NamedExport {
			ord: ord,
			symbol: Export::None,
			name: None,
		}
	}
	fn symbol_from_rva(&self, rva: &'a Rva) -> Export<'a> {
		if self.is_forwarded(*rva) {
			Export::Forward(self.view_.read_str(*rva).unwrap())
		}
		else {
			Export::Symbol(rva)
		}
	}
	/// Iterate over the ordinals of the exports.
	#[inline]
	pub fn iter(&self) -> ExportIterator {
		ExportIterator {
			exp: self,
			it: 0,
		}
	}
}

impl<'a, 'b> fmt::Display for ExportDirectory<'a, 'b> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "Exports for {}", self.name()));
		try!(writeln!(f, "  Characteristics: {:>08X}", self.image_.Characteristics));
		try!(writeln!(f, "  TimeDateStamp:   {}", self.image_.TimeDateStamp));
		try!(writeln!(f, "  Version:         {}.{}", self.image_.MajorVersion, self.image_.MinorVersion));
		try!(writeln!(f, "  OrdinalBase:     {}", self.image_.Base));
		try!(writeln!(f, "  # of Functions:  {}", self.image_.NumberOfFunctions));
		try!(writeln!(f, "  # of Names:      {}", self.image_.NumberOfNames));

		for ord in self.iter() {
			let name = self.name_from_ordinal(ord);
			match name.symbol {
				Export::None => (),
				_ => {
					try!(writeln!(f, "  {}", name));
				}
			}
		}
		Ok(())
	}
}

//----------------------------------------------------------------

pub trait PeExports {
	fn exports(&self) -> Option<ExportDirectory>;
}

impl<'a> PeExports for PeView<'a> {
	fn exports(&self) -> Option<ExportDirectory> {
		if let Some(datadir) = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXPORT) {
			if datadir.VirtualAddress != BADRVA {
				let image = self.read_struct::<ImageExportDirectory>(datadir.VirtualAddress).unwrap();
				Some(ExportDirectory {
					view_: self,
					datadir_: datadir,
					image_: image,
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

pub struct ExportIterator<'a: 'b, 'b> {
	exp: &'b ExportDirectory<'a, 'b>,
	it: u16,
}

impl<'a, 'b> Iterator for ExportIterator<'a, 'b> {
	type Item = u16;

	fn next(&mut self) -> Option<Self::Item> {
		if self.it as u32 >= self.exp.image_.NumberOfFunctions {
			None
		}
		else {
			let ord = self.it + (self.exp.image_.Base & 0xFFFF) as u16;
			self.it += 1;
			Some(ord)
		}
	}
}
