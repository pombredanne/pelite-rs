//! Structures as they are stored on disk.

#![allow(non_snake_case)]

#[cfg(windows)]
extern "C" {
	#[cfg(target_env = "msvc")]
	static __ImageBase: ImageDosHeader;
	#[cfg(target_env = "gnu")]
	static _image_base__: ImageDosHeader;
}

/// Get the base address of the module this code is linked with.
///
/// This uses a linker pseudovariable and is only available on windows platform.
#[cfg(all(windows, target_env = "msvc"))]
pub fn image_base() -> &'static ImageDosHeader {
	&__ImageBase
}
/// Get the base address of the module this code is linked with.
///
/// This uses a linker pseudovariable and is only available on windows platform.
#[cfg(all(windows, target_env = "gnu"))]
pub fn image_base() -> &'static ImageDosHeader {
	&_image_base__
}

//----------------------------------------------------------------

pub const IMAGE_DOS_HEADER_MAGIC: u16 = 0x5A4D;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageDosHeader {
	pub e_magic: u16,
	pub e_cblp: u16,
	pub e_cp: u16,
	pub e_crlc: u16,
	pub e_cparhdr: u16,
	pub e_minalloc: u16,
	pub e_maxalloc: u16,
	pub e_ss: u16,
	pub e_sp: u16,
	pub e_csum: u16,
	pub e_ip: u16,
	pub e_cs: u16,
	pub e_lfarlc: u16,
	pub e_ovno: u16,
	pub e_res: [u16; 4],
	pub e_oemid: u16,
	pub e_oeminfo: u16,
	pub e_res2: [u16; 10],
	pub e_lfanew: u32,
}

//----------------------------------------------------------------

pub const IMAGE_FILE_MACHINE_I386: u16  = 0x014c;
pub const IMAGE_FILE_MACHINE_IA64: u16  = 0x0200;
pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

pub const IMAGE_FILE_RELOCS_STRIPPED: u16         = 0x0001;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16        = 0x0002;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16      = 0x0004;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16     = 0x0008;
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: u16       = 0x0010;
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16     = 0x0020;
pub const IMAGE_FILE_BYTES_REVERSED_LO: u16       = 0x0080;
pub const IMAGE_FILE_32BIT_MACHINE: u16           = 0x0100;
pub const IMAGE_FILE_DEBUG_STRIPPED: u16          = 0x0200;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16       = 0x0800;
pub const IMAGE_FILE_SYSTEM: u16                  = 0x1000;
pub const IMAGE_FILE_DLL: u16                     = 0x2000;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16          = 0x4000;
pub const IMAGE_FILE_BYTES_REVERSED_HI: u16       = 0x8000;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageFileHeader {
	pub Machine: u16,
	pub NumberOfSections: u16,
	pub TimeDateStamp: u32,
	pub PointerToSymbolTable: u32,
	pub NumberOfSymbols: u32,
	pub SizeOfOptionalHeader: u16,
	pub Characteristics: u16,
}

//----------------------------------------------------------------

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageDataDirectory {
	pub VirtualAddress: u32,
	pub Size: u32,
}

pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize         = 0;
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: usize         = 1;
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize       = 2;
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize      = 3;
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: usize       = 4;
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize      = 5;
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: usize          = 6;
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: usize   = 7;
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: usize      = 8;
pub const IMAGE_DIRECTORY_ENTRY_TLS: usize            = 9;
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize    = 10;
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: usize   = 11;
pub const IMAGE_DIRECTORY_ENTRY_IAT: usize            = 12;
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: usize   = 13;
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14;

pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize   = 16;

//----------------------------------------------------------------

pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 0x10b;
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x20b;
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16  = 0x107;

pub const IMAGE_SUBSYSTEM_UNKNOWN: u16                  = 0;
pub const IMAGE_SUBSYSTEM_NATIVE: u16                   = 1;
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16              = 2;
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16              = 3;
pub const IMAGE_SUBSYSTEM_OS2_CUI: u16                  = 5;
pub const IMAGE_SUBSYSTEM_POSIX_CUI: u16                = 7;
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16           = 9;
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16          = 10;
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16  = 11;
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16       = 12;
pub const IMAGE_SUBSYSTEM_EFI_ROM: u16                  = 13;
pub const IMAGE_SUBSYSTEM_XBOX: u16                     = 14;
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16;

pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16          = 0x0040;
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16       = 0x0080;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16             = 0x0100;
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16          = 0x0200;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16                = 0x0400;
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16               = 0x0800;
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16            = 0x2000;
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 0x8000;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageOptionalHeader32 {
	pub Magic: u16,
	pub MajorLinkerVersion: u8,
	pub MinorLinkerVersion: u8,
	pub SizeOfCode: u32,
	pub SizeOfInitializedData: u32,
	pub SizeOfUninitializedData: u32,
	pub AddressOfEntryPoint: u32,
	pub BaseOfCode: u32,
	pub BaseOfData: u32,
	pub ImageBase: u32,
	pub SectionAlignment: u32,
	pub FileAlignment: u32,
	pub MajorOperatingSystemVersion: u16,
	pub MinorOperatingSystemVersion: u16,
	pub MajorImageVersion: u16,
	pub MinorImageVersion: u16,
	pub MajorSubsystemVersion: u16,
	pub MinorSubsystemVersion: u16,
	pub Win32VersionValue: u32,
	pub SizeOfImage: u32,
	pub SizeOfHeaders: u32,
	pub CheckSum: u32,
	pub Subsystem: u16,
	pub DllCharacteristics: u16,
	pub SizeOfStackReserve: u32,
	pub SizeOfStackCommit: u32,
	pub SizeOfHeapReserve: u32,
	pub SizeOfHeapCommit: u32,
	pub LoaderFlags: u32,
	pub NumberOfRvaAndSizes: u32,
	pub DataDirectory: [ImageDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageOptionalHeader64 {
	pub Magic: u16,
	pub MajorLinkerVersion: u8,
	pub MinorLinkerVersion: u8,
	pub SizeOfCode: u32,
	pub SizeOfInitializedData: u32,
	pub SizeOfUninitializedData: u32,
	pub AddressOfEntryPoint: u32,
	pub BaseOfCode: u32,
	pub ImageBase: u64,
	pub SectionAlignment: u32,
	pub FileAlignment: u32,
	pub MajorOperatingSystemVersion: u16,
	pub MinorOperatingSystemVersion: u16,
	pub MajorImageVersion: u16,
	pub MinorImageVersion: u16,
	pub MajorSubsystemVersion: u16,
	pub MinorSubsystemVersion: u16,
	pub Win32VersionValue: u32,
	pub SizeOfImage: u32,
	pub SizeOfHeaders: u32,
	pub CheckSum: u32,
	pub Subsystem: u16,
	pub DllCharacteristics: u16,
	pub SizeOfStackReserve: u64,
	pub SizeOfStackCommit: u64,
	pub SizeOfHeapReserve: u64,
	pub SizeOfHeapCommit: u64,
	pub LoaderFlags: u32,
	pub NumberOfRvaAndSizes: u32,
	pub DataDirectory: [ImageDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}

//----------------------------------------------------------------

pub const IMAGE_NT_HEADERS_SIGNATURE: u32 = 0x00004550;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageNtHeaders32 {
	pub Signature: u32,
	pub FileHeader: ImageFileHeader,
	pub OptionalHeader: ImageOptionalHeader32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageNtHeaders64 {
	pub Signature: u32,
	pub FileHeader: ImageFileHeader,
	pub OptionalHeader: ImageOptionalHeader64,
}

//----------------------------------------------------------------

pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;

pub const IMAGE_SCN_TYPE_NO_PAD: u32            = 0x00000008;
pub const IMAGE_SCN_CNT_CODE: u32               = 0x00000020;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32   = 0x00000040;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x00000080;
pub const IMAGE_SCN_LNK_OTHER: u32              = 0x00000100;
pub const IMAGE_SCN_LNK_INFO: u32               = 0x00000200;
pub const IMAGE_SCN_LNK_REMOVE: u32             = 0x00000800;
pub const IMAGE_SCN_LNK_COMDAT: u32             = 0x00001000;
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: u32      = 0x00004000;
pub const IMAGE_SCN_GPREL: u32                  = 0x00008000;
pub const IMAGE_SCN_MEM_PURGEABLE: u32          = 0x00020000;
pub const IMAGE_SCN_MEM_LOCKED: u32             = 0x00040000;
pub const IMAGE_SCN_MEM_PRELOAD: u32            = 0x00080000;
pub const IMAGE_SCN_ALIGN_1BYTES: u32           = 0x00100000;
pub const IMAGE_SCN_ALIGN_2BYTES: u32           = 0x00200000;
pub const IMAGE_SCN_ALIGN_4BYTES: u32           = 0x00300000;
pub const IMAGE_SCN_ALIGN_8BYTES: u32           = 0x00400000;
pub const IMAGE_SCN_ALIGN_16BYTES: u32          = 0x00500000;
pub const IMAGE_SCN_ALIGN_32BYTES: u32          = 0x00600000;
pub const IMAGE_SCN_ALIGN_64BYTES: u32          = 0x00700000;
pub const IMAGE_SCN_ALIGN_128BYTES: u32         = 0x00800000;
pub const IMAGE_SCN_ALIGN_256BYTES: u32         = 0x00900000;
pub const IMAGE_SCN_ALIGN_512BYTES: u32         = 0x00A00000;
pub const IMAGE_SCN_ALIGN_1024BYTES: u32        = 0x00B00000;
pub const IMAGE_SCN_ALIGN_2048BYTES: u32        = 0x00C00000;
pub const IMAGE_SCN_ALIGN_4096BYTES: u32        = 0x00D00000;
pub const IMAGE_SCN_ALIGN_8192BYTES: u32        = 0x00E00000;
pub const IMAGE_SCN_LNK_NRELOC_OVFL: u32        = 0x01000000;
pub const IMAGE_SCN_MEM_DISCARDABLE: u32        = 0x02000000;
pub const IMAGE_SCN_MEM_NOT_CACHED: u32         = 0x04000000;
pub const IMAGE_SCN_MEM_NOT_PAGED: u32          = 0x08000000;
pub const IMAGE_SCN_MEM_SHARED: u32             = 0x10000000;
pub const IMAGE_SCN_MEM_EXECUTE: u32            = 0x20000000;
pub const IMAGE_SCN_MEM_READ: u32               = 0x40000000;
pub const IMAGE_SCN_MEM_WRITE: u32              = 0x80000000;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageSectionHeader {
	pub Name: [u8; IMAGE_SIZEOF_SHORT_NAME],
	pub VirtualSize: u32,
	pub VirtualAddress: u32,
	pub SizeOfRawData: u32,
	pub PointerToRawData: u32,
	pub PointerToRelocations: u32,
	pub PointerToLinenumbers: u32,
	pub NumberOfRelocations: u16,
	pub NumberOfLinenumbers: u16,
	pub Characteristics: u32,
}

//----------------------------------------------------------------

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageExportDirectory {
	pub Characteristics: u32,
	pub TimeDateStamp: u32,
	pub MajorVersion: u16,
	pub MinorVersion: u16,
	pub Name: u32,
	pub Base: u32,
	pub NumberOfFunctions: u32,
	pub NumberOfNames: u32,
	pub AddressOfFunctions: u32,     // RVA from base of image
	pub AddressOfNames: u32,         // RVA from base of image
	pub AddressOfNameOrdinals: u32,  // RVA from base of image
}

//----------------------------------------------------------------

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageImportDescriptor {
	pub OriginalFirstThunk: u32,
	pub TimeDateStamp: u32,
	pub ForwarderChain: u32,
	pub Name: u32,
	pub FirstThunk: u32,
}

pub const IMAGE_ORDINAL_FLAG32: u32 = 0x80000000;
pub const IMAGE_ORDINAL_FLAG64: u64 = 0x8000000000000000;

//----------------------------------------------------------------

pub const RT_CURSOR: u16       = 1;
pub const RT_BITMAP: u16       = 2;
pub const RT_ICON: u16         = 3;
pub const RT_MENU: u16         = 4;
pub const RT_DIALOG: u16       = 5;
pub const RT_STRING: u16       = 6;
pub const RT_FONTDIR: u16      = 7;
pub const RT_FONT: u16         = 8;
pub const RT_ACCELERATOR: u16  = 9;
pub const RT_RCDATA: u16       = 10;
pub const RT_MESSAGETABLE: u16 = 11;
pub const RT_GROUP_CURSOR: u16 = 12;
pub const RT_GROUP_ICON: u16   = 14;
pub const RT_VERSION: u16      = 16;
pub const RT_DLGINCLUDE: u16   = 17;
pub const RT_PLUGPLAY: u16     = 19;
pub const RT_VXD: u16          = 20;
pub const RT_ANICURSOR: u16    = 21;
pub const RT_ANIICON: u16      = 22;
pub const RT_HTML: u16         = 23;
pub const RT_MANIFEST: u16     = 24;

pub const RSRC_TYPES: &'static [Option<&'static str>] = &[
	/* 0*/ None, Some("Cursor"), Some("Bitmap"), Some("Icon"), Some("Menu"),
	/* 5*/ Some("Dialog"), Some("String"), Some("FontDir"), Some("Font"), Some("Accelerator"),
	/*10*/ Some("RCData"), Some("MessageTable"), Some("Group Cursor"), None, Some("Group Icon"),
	/*15*/ None, Some("Version"), Some("DlgInclude"), None, Some("PlugPlay"),
	/*20*/ Some("VXD"), Some("AniCursor"), Some("AniIcon"), Some("HTML"), Some("Manifest"),
];

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageResourceDirectory {
	pub Characteristics: u32,
	pub TimeDateStamp: u32,
	pub MajorVersion: u16,
	pub MinorVersion: u16,
	pub NumberOfNamedEntries: u16,
	pub NumberOfIdEntries: u16,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageResourceDirectoryEntry {
	// High bit set means the lower 31 bits are an RVA to its name string otherwise this is a 16 bit WORD id
	// Name string is encoded in WORDs and is prefixed with a WORD indicating its length (in WORDs)
	pub Name: u32,
	// High bit set means this is offset points to an ImageResourceDirectory otherwise an ImageResourceDataEntry
	pub Offset: u32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageResourceDataEntry {
	pub OffsetToData: u32,
	pub Size: u32,
	pub CodePage: u32,
	pub Reserved: u32,
}

//----------------------------------------------------------------

pub const IMAGE_REL_BASED_ABSOLUTE: u8 = 0;
pub const IMAGE_REL_BASED_HIGH: u8 = 1;
pub const IMAGE_REL_BASED_LOW: u8 = 2;
pub const IMAGE_REL_BASED_HIGHLOW: u8 = 3;
pub const IMAGE_REL_BASED_HIGHADJ: u8 = 4;
pub const IMAGE_REL_BASED_MIPSJMPADDR: u8 = 5;
pub const IMAGE_REL_BASED_MIPSJMPADDR16: u8 = 9;
pub const IMAGE_REL_BASED_IA64IMM64: u8 = 9;
pub const IMAGE_REL_BASED_DIR64: u8 = 10;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageBaseRelocation {
	pub VirtualAddress: u32,
	pub SizeOfBlock: u32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct ImageBaseRelocBlock {
	// bit field:
	// |0123|456789ABCDEF|
	// |Type|   Offset   |
	pub TypeAndOffset: u16,
}
