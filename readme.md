PeLite Library
==============

Lightweight, memory-safe, zero-allocation library for reading and navigating PE binaries.

Design
------

The purpose of this library is inspecting PE binaries (whether on disk or already loaded in memory). While it can correctly map binaries to memory it is not intended for actual execution of the image.

A trade-off was made to not unify the 32bit (PE32) and 64bit (PE32+) formats for two reasons:

* There are small but incompatible differences, which would add overhead by requiring constant matching even if at source code level the match arms look identical.

* Most of the time you know (at build time) what format you're working with anyway.

This makes it rather awkward to work with both formats together transparently.

Note that while the correct name is PE32+, the name PE64 is used as it is a valid identifier; they are otherwise synonymous.

Corrupt PE files may panic, but is not guaranteed if the result looks as if it's valid.

ELF format is not supported and not planned. There is an [elf library crate](https://crates.io/crates/elf) but its design has a different focus.

Documentation
-------------

For now they can be found on [crates.fyi](https://crates.fyi/crates/pelite/).

Usage
-----

This library can be found on [crates.io](https://crates.io/crates/pelite). In your Cargo.toml put

```
[dependencies]
pelite = "0.2"
```

The following code samples all assume PE32+, more examples can be found under `examples/`.

When working with binaries on disk, they must first be mapped correctly to memory. This makes them take more space in memory but makes the code more simple and efficient (as they only need to deal with correctly mapped images).

```rust
extern crate pelite;
use std::path::Path;
use pelite::pe64;

fn main() {
	// Load a file from disk, may fail
	let file = pe64::pefile::PeFile::open(Path::new("tests/bin/dummy64.dll")).unwrap();

	// Once mapped get the PeView to create a read-only view to inspect the data structures
	let view = file.view();

	// ...
}
```

When working with binaries already mapped to memory by the OS (Windows in this case) you can create `PeView`s directly through the unsafe `PeView::module` constructor.

Depending on the target platform this means that you don't know if you'll be executed as 32bit or 64bit process at the time you're writing your source code but it is known at build time. For this purpose there is `pelite::pe` which is a reexport of `pelite::pe64` on 64bit and `pelite::pe32` on 32bit targets. This reexport is only available for windows targets.

```rust
extern crate pelite;

// Select the appropriate 32bit or 64bit module at build time. If this symbol is missing you are not building for a windows target.
use pelite::pe;

fn main() {
	// Get a pointer to the base of the image mapped in memory
	// (Eg return value of LoadLibrary or GetModuleHandle, either is outside the scope of this example)
	let hmodule: *const u8;

	// You can get the image base this code is executing in like this
	hmodule = pe::image::image_base() as *const _ as *const u8;

	// Create the PeView for it, no validation is done at this time
	let view = unsafe { pe::peview::PeView::module(hmodule) };

	// ...
}
```

Those are the two main ways to create a `PeView` which can then be used to inspect the data structures inside.

At this point the library supports reading the following: headers, exports, imports, relocations and resources. See `examples/` for more specific examples.

License
-------

MIT, see license.txt
