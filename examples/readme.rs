#![allow(unused_variables)]
extern crate pelite;

use std::path::Path;
use pelite::pe64;

fn example1() {
	// Load a file from disk, may fail
	let file = pe64::pefile::PeFile::open(Path::new("tests/bin/dummy64.dll")).unwrap();

	// Once mapped get the PeView to create a read-only view to inspect the data structures
	let view = file.view();

	// ...
}

// Select the appropriate 32bit or 64bit module at build time. If this symbol is missing you are not building for a windows target.
use pelite::pe;

fn example2() {
	// Get a pointer to the base of the image mapped in memory
	// (Eg return value of LoadLibrary or GetModuleHandle, either is outside the scope of this example)
	let hmodule: *const u8;

	// You can get the image base this code is executing in like this
	hmodule = pe::image::image_base() as *const _ as *const u8;

	// Create the PeView for it, no validation is done at this time
	let view = unsafe { pe::peview::PeView::module(hmodule) };

	// ...
}

// Keep the examples from the readme buildable
fn main() {
	example1();
	example2();
}
