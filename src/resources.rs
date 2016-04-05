//! PE resources.
//!
//! It is known how the data is structured but I'm unsure how this is actually used.
//! Therefore this code is a very thin wrapper around the structures in the resources.

use std::{slice, fmt, mem};
use std::fmt::Write;

use super::image::*;

//----------------------------------------------------------------

/// Resources filesystem.
pub struct Resources<'a> {
	data: &'a [u8],
	vbase: u32,
}

impl<'a> Resources<'a> {
	/// Interpret memory as a resources format.
	///
	/// # Parameters
	///
	/// * `data`
	///
	///   Memory to interpret.
	///
	/// * `vbase`
	///
	///   All offsets _except_ the final `ImageResourceDataEntry::OffsetToData` are relative to the resource directory.
	///   This value is subtracted from `OffsetToData` before being used as an offset in this resource directory.
	///   Just... Why would you do this?
	///
	/// # Remarks
	///
	/// No validation is done ahead of time.
	pub fn new(data: &'a [u8], vbase: u32) -> Resources<'a> {
		Resources {
			data: data,
			vbase: vbase,
		}
	}
	/// Start by getting the root directory entry.
	pub fn root(&self) -> ResourceDirectoryEntry {
		const ROOT_ENTRY: &'static ImageResourceDirectoryEntry = &ImageResourceDirectoryEntry { Name: 0, Offset: 0x80000000 };
		ResourceDirectoryEntry {
			resrc_: self,
			image_: ROOT_ENTRY,
		}
	}
	fn read_slice(&self, off: usize, len: usize) -> &[u8] {
		// Panics on invalid input; this is desired behaviour as it indicates corruption
		&self.data[off .. off + len]
	}
	fn read_str(&self, off: usize) -> &[u16] {
		// Reads the resource names which are utf16
		let words = *self.read::<u16>(off) as usize;
		let nameptr = self.read_slice(off + 2, words * 2).as_ptr() as *const u16;
		unsafe { slice::from_raw_parts(nameptr, words) }
	}
	fn read<T>(&self, off: usize) -> &T {
		unsafe { &*(self.read_slice(off, mem::size_of::<T>()).as_ptr() as *const _) }
	}
}

impl<'a> fmt::Display for Resources<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "Resources"));
		write!(f, "{}", self.root())
	}
}

//----------------------------------------------------------------

/// Represent a resource name.
pub enum ResourceName<'a> {
	/// A u16 resource ID.
	Id(u16),
	/// UTF-16 named resource.
	Name(&'a [u16]),
}

impl<'a> fmt::Display for ResourceName<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ResourceName::Id(id) => {
				write!(f, "#{}", id)
			},
			ResourceName::Name(name) => {
				// FIXME! This allocation is unnecessary, but the required utf16 decoder isn't stable (yet).
				write!(f, "{}", String::from_utf16_lossy(name))
			},
		}
	}
}

/// Resource directory entries are either further subdirectories or data entries.
pub enum ResourceEntry<'a> {
	Directory(ResourceDirectory<'a>),
	DataEntry(ResourceDataEntry<'a>),
}

/// Directory entry.
pub struct ResourceDirectoryEntry<'a> {
	resrc_: &'a Resources<'a>,
	image_: &'a ImageResourceDirectoryEntry,
}

impl<'a> ResourceDirectoryEntry<'a> {
	/// Get the resources being worked with.
	pub fn resources(&self) -> &Resources {
		self.resrc_
	}
	/// Get the underlying directory entry image.
	pub fn image(&self) -> &ImageResourceDirectoryEntry {
		&self.image_
	}
	/// Get the name for this entry.
	pub fn name(&self) -> ResourceName {
		if self.image_.Name & 0x80000000 != 0 {
			let offset = (self.image_.Name & !0x80000000) as usize;
			let name = self.resrc_.read_str(offset);
			ResourceName::Name(name)
		}
		else {
			ResourceName::Id((self.image_.Name & 0xFFFF) as u16)
		}
	}
	/// Is this entry a subdirectory?
	pub fn is_dir(&self) -> bool {
		self.image_.Offset & 0x80000000 != 0
	}
	/// Interpret this entry as a subdirectory.
	pub fn as_dir(&self) -> Option<ResourceDirectory> {
		if self.is_dir() {
			let offset = (self.image_.Offset & !0x80000000) as usize;
			// Ensures there's at least enough to read the directory image
			let image = self.resrc_.read::<ImageResourceDirectory>(offset);
			// Ensures the entire directory image and its entries can be read
			let bytes = mem::size_of::<ImageResourceDirectory>() + (image.NumberOfNamedEntries as usize + image.NumberOfIdEntries as usize) * mem::size_of::<ImageResourceDirectoryEntry>();
			let image = unsafe { &*(self.resrc_.read_slice(offset, bytes).as_ptr() as *const ImageResourceDirectory) };
			// This is a valid directory contained within the resources
			Some(ResourceDirectory {
				entry_: self,
				image_: image,
			})
		}
		else {
			None
		}
	}
	/// Interpret this entry as a data entry.
	pub fn as_data(&self) -> Option<ResourceDataEntry> {
		if !self.is_dir() {
			let offset = self.image_.Offset as usize;
			let image = self.resrc_.read::<ImageResourceDataEntry>(offset);
			Some(ResourceDataEntry {
				entry_: self,
				image_: image,
			})
		}
		else {
			None
		}
	}
	/// Get the entry as either subdirectory or data entry.
	pub fn entry(&self) -> ResourceEntry {
		// These unwrap()s should get optimized out.
		if self.is_dir() {
			ResourceEntry::Directory(self.as_dir().unwrap())
		}
		else {
			ResourceEntry::DataEntry(self.as_data().unwrap())
		}
	}
}

impl<'a> fmt::Display for ResourceDirectoryEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fn rec(f: &mut fmt::Formatter, path: Option<&String>, e: &ResourceDirectoryEntry) -> fmt::Result {
			// Format append current entry name to the path so far
			let str = match path {
				Some(path) => {
					format!("{}/{}", path, e.name())
				},
				None => {
					String::new()
				}
			};
			// Print the entry information
			match e.entry() {
				ResourceEntry::Directory(dir) => {
					try!(writeln!(f, "DIR {}/", str));
					try!(write!(f, "{}", dir));
					// Recursively print all its children
					for it in dir.iter() {
						try!(rec(f, Some(&str), &it));
					}
					Ok(())
				},
				ResourceEntry::DataEntry(data) => {
					try!(writeln!(f, "DATA {}", str));
					write!(f, "{}", data)
				},
			}
		}
		rec(f, None, self)
	}
}

//----------------------------------------------------------------

/// A resource directory.
pub struct ResourceDirectory<'a> {
	entry_: &'a ResourceDirectoryEntry<'a>,
	image_: &'a ImageResourceDirectory,
}

impl<'a> ResourceDirectory<'a> {
	/// Get the resources being worked with.
	pub fn resources(&self) -> &Resources {
		self.entry_.resrc_
	}
	/// Get the directory entry for this subdirectory.
	pub fn entry(&self) -> &ResourceDirectoryEntry {
		self.entry_
	}
	/// Get the underlying directory image.
	pub fn image(&self) -> &ImageResourceDirectory {
		self.image_
	}
	/// Find a child entry by name. Not very efficient due to String conversions...
	pub fn find(&self, name: &str) -> Option<ResourceDirectoryEntry> {
		self.iter().find(|e| {
			// Allocates a new String for every compare :(
			format!("{}", e.name()) == name
		})
	}
	/// Iterate over the child entries.
	pub fn iter(&self) -> ResourceDirectoryIterator {
		ResourceDirectoryIterator {
			dir: self,
			it: 0,
		}
	}
	fn entries(&self) -> &[ImageResourceDirectoryEntry] {
		unsafe {
			let ptr = (self.image_ as *const _).offset(1) as *const ImageResourceDirectoryEntry;
			let len = self.image_.NumberOfNamedEntries as usize + self.image_.NumberOfIdEntries as usize;
			slice::from_raw_parts(ptr, len)
		}
	}
}

impl<'a> fmt::Display for ResourceDirectory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "  Characteristics: {}", self.image_.Characteristics));
		try!(writeln!(f, "  TimeDateStamp:   {}", self.image_.TimeDateStamp));
		try!(writeln!(f, "  Version:         {}.{}", self.image_.MajorVersion, self.image_.MinorVersion));
		try!(writeln!(f, "  NumberOfEntries: {}", self.entries().len()));
		Ok(())
	}
}

//----------------------------------------------------------------

pub struct ResourceDirectoryIterator<'a> {
	dir: &'a ResourceDirectory<'a>,
	it: usize,
}

impl<'a> Iterator for ResourceDirectoryIterator<'a> {
	type Item = ResourceDirectoryEntry<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		// This felt nice to write :)
		self.dir.entries().get(self.it).map(|dir_entry| {
			self.it += 1;
			ResourceDirectoryEntry {
				resrc_: self.dir.entry_.resrc_,
				image_: dir_entry,
			}
		})
	}
}

//----------------------------------------------------------------

/// A resource data entry.
pub struct ResourceDataEntry<'a> {
	entry_: &'a ResourceDirectoryEntry<'a>,
	image_: &'a ImageResourceDataEntry,
}

impl<'a> ResourceDataEntry<'a> {
	/// Get the resources being worked with.
	pub fn resources(&self) -> &Resources {
		self.entry_.resrc_
	}
	/// Get the directory entry for this data entry.
	pub fn entry(&self) -> &ResourceDirectoryEntry {
		self.entry_
	}
	/// Get the underlying data entry image.
	pub fn image(&self) -> &ImageResourceDataEntry {
		self.image_
	}
	/// Get the resource data as a byte slice.
	pub fn data(&self) -> &[u8] {
		let offset = self.image_.OffsetToData as usize - self.entry_.resrc_.vbase as usize;
		self.entry_.resrc_.read_slice(offset, self.image_.Size as usize)
	}
}

impl<'a> fmt::Display for ResourceDataEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "  OffsetToData:    {:>08X}", self.image_.OffsetToData));
		try!(writeln!(f, "  Size:            {:>08X}", self.image_.Size));
		try!(writeln!(f, "  CodePage:        {}", self.image_.CodePage));
		Ok(())
	}
}
