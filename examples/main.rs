#![allow(unused_variables)]
extern crate pelite;

// When working with executable files on disk, you must known beforehand if you want to work with PE32 or PE64 (or both).
//
// When working with modules mapped by the OS in the current process, you can use the reexport `pelite::pe` to automatically select the right version.
// This is windows only, as it makes no sense to work with 'PE modules mapped by the OS' when that OS is not windows.
//
// The design reason for not unifying PE32 and PE64 is that while they may look similar, they're incompatible in some crucial ways.
// And I do not want to force overhead of matching between the two when most of the time you already know what you're working with.
use pelite::pe;

// You must import what features of the PE format you wish to work with, here we choose the PE imports.
use pelite::pe::imports::PeViewImports;

fn main() {
	// Create a view in the module this code is currently executing in:
	// If the compiler complains about missing symbols, you may not be running windows, or using the wrong PE pointer width version.
	let view = pe::peview::PeView::new();

	// Access the imports of the module.
	// Note that not every module has imports (although this is pretty rare :)
	let imports = view.imports().unwrap();

	// All we can do with imports is to iterate over the imported module descriptors.
	// Here `desc` has the type `pe::imports::ImportDescriptor`.
	for desc in imports.iter() {
		// This descriptor imports from this dll name.
		// This value is typically passed straight to LoadLibrary.
		let dll_name = desc.dll_name();

		// For every imported module there's two tables:
		//
		// INT: Import Name Table, this let's you know the imported symbol name or ordinal.
		//      Get an iterator over these with desc.int_iter()
		//
		// IAT: Import Address Table, contains the actual addresses of the imported symbols as used by the module.
		//      Get an iterator over these with desc.iat_iter()
		//
		// You can zip these together.
		//
		// `name` has type `pe::imports::ImportedSymbol`, `addr` has type `&pe::image::Va`
		for (name, addr) in desc.int_iter().zip(desc.iat_iter()) {

		}
	}
}
