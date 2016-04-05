extern crate pelite;
use std::path::Path;
use pelite::pe32::peview::PeView;
use pelite::pe32::pefile::PeFile;
use pelite::pe32::exports::PeViewExports;
use pelite::pe32::imports::PeViewImports;
use pelite::pe32::relocs::PeViewRelocs;
use pelite::pe32::resources::PeViewResources;

#[test]
fn test_dummyd_dll() {
	let file = PeFile::open(Path::new("tests\\bin\\dummyd.dll")).unwrap();
	run_tests(&file.view());
}
#[test]
fn test_dummy_dll() {
	let file = PeFile::open(Path::new("tests\\bin\\dummy.dll")).unwrap();
	run_tests(&file.view());
}

fn run_tests(view: &PeView) {
	println!("{}", view.imports().unwrap());
	println!("{}", view.exports().unwrap());
	println!("{}", view.resources().unwrap());
	println!("{}", view.relocs().unwrap());
}
