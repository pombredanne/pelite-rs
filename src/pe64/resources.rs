//! PE resources.
//!
//! See `pelite::resources` for more info.

use super::image::*;
use super::peview::PeView;
use resources::Resources;

pub trait PeViewResources {
	fn resources(&self) -> Option<Resources>;
}

impl<'a> PeViewResources for PeView<'a> {
	fn resources(&self) -> Option<Resources> {
		if let Some(datadir) = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_RESOURCE) {
			if datadir.VirtualAddress != BADRVA {
				let resrc = self.read_slice::<u8>(datadir.VirtualAddress, datadir.Size as usize).unwrap();
				return Some(Resources::new(resrc, datadir.VirtualAddress));
			}
		}
		return None;
	}
}
