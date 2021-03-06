extern crate pelite;
use std::path::Path;
use pelite::pe64::peview::PeView;
use pelite::pe64::pefile::PeFile;
use pelite::pe64::exports::PeExports;
use pelite::pe64::imports::PeImports;
use pelite::pe64::relocs::PeRelocs;
use pelite::pe64::resources::PeResources;

#[test]
fn test_dummy64d_dll() {
	let file = PeFile::open(Path::new("tests\\bin\\dummy64d.dll")).unwrap();
	run_tests(&file.view());
}
#[test]
fn test_dummy64_dll() {
	let file = PeFile::open(Path::new("tests\\bin\\dummy64.dll")).unwrap();
	run_tests(&file.view());
}

fn run_tests(view: &PeView) {
	println!("{}", view.imports().unwrap());
	println!("{}", view.exports().unwrap());
	println!("{}", view.resources().unwrap());
	println!("{}", view.relocs().unwrap());
}
