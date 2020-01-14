use num_enum::TryFromPrimitive;
use std::fmt;
use std::fmt::Display;

/// Length of ELF file header platform-independent identification fields
pub const EI_NIDENT: usize = 16;
/// ELF magic number byte 1
pub const ELFMAG0: u8 = 0x7f;
/// ELF magic number byte 2
pub const ELFMAG1: u8 = 0x45;
/// ELF magic number byte 3
pub const ELFMAG2: u8 = 0x4c;
/// ELF magic number byte 4
pub const ELFMAG3: u8 = 0x46;
/// Location of ELF class field in ELF file header ident array
pub const EI_CLASS: usize = 4;
/// Location of data format field in ELF file header ident array
pub const EI_DATA: usize = 5;
/// Location of ELF version field in ELF file header ident array
pub const EI_VERSION: usize = 6;
/// Location of OS ABI field in ELF file header ident array
pub const EI_OSABI: usize = 7;
/// Location of ABI version field in ELF file header ident array
pub const EI_ABIVERSION: usize = 8;

/// Represents the ELF file class (32-bit vs 64-bit).
///
/// Represented as the fourth byte in an ELF file and named EI_CLASS in C code.
#[derive(Copy, Clone, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ElfClass {
    /// Format for 32-bit ELF files. Named ELFCLASS32 in C code.
    Format32 = 1,
    /// Format for 64-bit ELF files. Named ELFCLASS64 in C code.
    Format64 = 2,
}

/// The endianness encoding of an ELF file (LSB or MSB).
///
/// Represented as the fifth byte in an ELF file and named EI_DATA in C code.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ElfEndianness {
    /// Least significant byte first endianness. Named ELFDATA2LSB in C code.
    Lsb = 1,
    /// Most significant byte first endianness. Named ELFDATA2MSB in C code.
    Msb = 2,
}

/// Represents the ELF file OS ABI
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct OSABI(pub u8);
/// Defaults to Unix System V
pub const ELFOSABI_NONE: OSABI = OSABI(0);
/// Unix System V
pub const ELFOSABI_SYSV: OSABI = OSABI(0);
/// HP-UX
pub const ELFOSABI_HPUX: OSABI = OSABI(1);
/// NetBSD
pub const ELFOSABI_NETBSD: OSABI = OSABI(2);
/// Linux with GNU extensions
pub const ELFOSABI_LINUX: OSABI = OSABI(3);
/// Solaris
pub const ELFOSABI_SOLARIS: OSABI = OSABI(6);
/// AIX
pub const ELFOSABI_AIX: OSABI = OSABI(7);
/// SGI Irix
pub const ELFOSABI_IRIX: OSABI = OSABI(8);
/// FreeBSD
pub const ELFOSABI_FREEBSD: OSABI = OSABI(9);
/// Compaq TRU64 UNIX
pub const ELFOSABI_TRU64: OSABI = OSABI(10);
/// Novell Modesto
pub const ELFOSABI_MODESTO: OSABI = OSABI(11);
/// OpenBSD
pub const ELFOSABI_OPENBSD: OSABI = OSABI(12);

impl fmt::Debug for OSABI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Display for OSABI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            ELFOSABI_SYSV => "UNIX System V",
            ELFOSABI_HPUX => "HP-UX",
            ELFOSABI_NETBSD => "NetBSD",
            ELFOSABI_LINUX => "Linux with GNU extensions",
            ELFOSABI_SOLARIS => "Solaris",
            ELFOSABI_AIX => "AIX",
            ELFOSABI_IRIX => "SGI Irix",
            ELFOSABI_FREEBSD => "FreeBSD",
            ELFOSABI_TRU64 => "Compaq TRU64 UNIX",
            ELFOSABI_MODESTO => "Novell Modesto",
            ELFOSABI_OPENBSD => "OpenBSD",
            _ => "Unknown",
        };
        write!(f, "{}", str)
    }
}

/// Represents the ELF file type (object, executable, shared lib, core)
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum ElfFileType {
    /// No file type. Named ET_NONE in C code.
    None = 0,
    /// Relocatable object file. Named ET_REL in C code.
    RelocatableObject = 1,
    /// Executable file. Named ET_EXEC in C code.
    Executable = 2,
    /// Shared library. Named ET_DYN in C code.
    SharedLibrary = 3,
    /// Core file. Named ET_CORE in C code.
    Core = 4,
}

/// Represents the ELF file machine architecture
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum ElfCpuArchitecture {
    EM_NONE = 0,
    EM_M32 = 1,
    EM_SPARC = 2,
    EM_386 = 3,
    EM_68K = 4,
    EM_88K = 5,
    EM_860 = 7,
    EM_MIPS = 8,
    EM_S370 = 9,
    EM_MIPS_RS3_LE = 10,
    EM_PARISC = 15,
    EM_VPP500 = 17,
    EM_SPARC32PLUS = 18,
    EM_960 = 19,
    EM_PPC = 20,
    EM_PPC64 = 21,
    EM_S390 = 22,
    EM_V800 = 36,
    EM_FR20 = 37,
    EM_RH32 = 38,
    EM_RCE = 39,
    EM_ARM = 40,
    EM_FAKE_ALPHA = 41,
    EM_SH = 42,
    EM_SPARCV9 = 43,
    EM_TRICORE = 44,
    EM_ARC = 45,
    EM_H8_300 = 46,
    EM_H8_300H = 47,
    EM_H8S = 48,
    EM_H8_500 = 49,
    EM_IA_64 = 50,
    EM_MIPS_X = 51,
    EM_COLDFIRE = 52,
    EM_68HC12 = 53,
    EM_MMA = 54,
    EM_PCP = 55,
    EM_NCPU = 56,
    EM_NDR1 = 57,
    EM_STARCORE = 58,
    EM_ME16 = 59,
    EM_ST100 = 60,
    EM_TINYJ = 61,
    EM_X86_64 = 62,
    EM_PDSP = 63,
    EM_FX66 = 66,
    EM_ST9PLUS = 67,
    EM_ST7 = 68,
    EM_68HC16 = 69,
    EM_68HC11 = 70,
    EM_68HC08 = 71,
    EM_68HC05 = 72,
    EM_SVX = 73,
    EM_ST19 = 74,
    EM_VAX = 75,
    EM_CRIS = 76,
    EM_JAVELIN = 77,
    EM_FIREPATH = 78,
    EM_ZSP = 79,
    EM_MMIX = 80,
    EM_HUANY = 81,
    EM_PRISM = 82,
    EM_AVR = 83,
    EM_FR30 = 84,
    EM_D10V = 85,
    EM_D30V = 86,
    EM_V850 = 87,
    EM_M32R = 88,
    EM_MN10300 = 89,
    EM_MN10200 = 90,
    EM_PJ = 91,
    EM_OPENRISC = 92,
    EM_ARC_A5 = 93,
    EM_XTENSA = 94,
    EM_AARCH64 = 183,
    EM_TILEPRO = 188,
    EM_MICROBLAZE = 189,
    EM_TILEGX = 191,
}

/// Encapsulates the contents of the ELF File Header
///
/// The ELF File Header starts off every ELF file and both identifies the
/// file contents and informs how to interpret said contents. This includes
/// the width of certain fields (32-bit vs 64-bit), the data endianness, the
/// file type, and more.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FileHeader {
    /// 32-bit vs 64-bit
    pub class: ElfClass,
    /// little vs big endian
    pub endianness: ElfEndianness,
    /// OS ABI
    pub osabi: OSABI,
    /// Version of the OS ABI
    pub abiversion: u8,
    /// ELF file type
    pub elftype: ElfFileType,
    /// Target machine architecture
    pub cpu_architecture: ElfCpuArchitecture,
    /// Virtual address of program entry point
    pub entry: u64,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File Header for {:?} {} Elf {} for {} {}",
            self.class, self.endianness, self.elftype, self.osabi, self.cpu_architecture
        )
    }
}

/// Represents ELF Program Header flags
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ProgFlag(pub u32);
pub const PF_NONE: ProgFlag = ProgFlag(0);
/// Executable program segment
pub const PF_X: ProgFlag = ProgFlag(1);
/// Writable program segment
pub const PF_W: ProgFlag = ProgFlag(2);
/// Readable program segment
pub const PF_R: ProgFlag = ProgFlag(4);

impl fmt::Debug for ProgFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Display for ProgFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (self.0 & PF_R.0) != 0 {
            write!(f, "R")?;
        } else {
            write!(f, " ")?;
        }
        if (self.0 & PF_W.0) != 0 {
            write!(f, "W")?;
        } else {
            write!(f, " ")?;
        }
        if (self.0 & PF_X.0) != 0 {
            write!(f, "E")
        } else {
            write!(f, " ")
        }
    }
}

/// Represents ELF Program Header type
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ProgType(pub u32);
/// Program header table entry unused
pub const PT_NULL: ProgType = ProgType(0);
/// Loadable program segment
pub const PT_LOAD: ProgType = ProgType(1);
/// Dynamic linking information
pub const PT_DYNAMIC: ProgType = ProgType(2);
/// Program interpreter
pub const PT_INTERP: ProgType = ProgType(3);
/// Auxiliary information
pub const PT_NOTE: ProgType = ProgType(4);
/// Unused
pub const PT_SHLIB: ProgType = ProgType(5);
/// The program header table
pub const PT_PHDR: ProgType = ProgType(6);
/// Thread-local storage segment
pub const PT_TLS: ProgType = ProgType(7);
/// GCC .eh_frame_hdr segment
pub const PT_GNU_EH_FRAME: ProgType = ProgType(0x6474_e550);
/// Indicates stack executability
pub const PT_GNU_STACK: ProgType = ProgType(0x6474_e551);
/// Read-only after relocation
pub const PT_GNU_RELRO: ProgType = ProgType(0x6474_e552);

impl fmt::Debug for ProgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Display for ProgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            PT_NULL => "NULL",
            PT_LOAD => "LOAD",
            PT_DYNAMIC => "DYNAMIC",
            PT_INTERP => "INTERP",
            PT_NOTE => "NOTE",
            PT_SHLIB => "SHLIB",
            PT_PHDR => "PHDR",
            PT_TLS => "TLS",
            PT_GNU_EH_FRAME => "GNU_EH_FRAME",
            PT_GNU_STACK => "GNU_STACK",
            PT_GNU_RELRO => "GNU_RELRO",
            _ => "Unknown",
        };
        write!(f, "{}", str)
    }
}

/// Encapsulates the contents of an ELF Program Header
///
/// The program header table is an array of program header structures describing
/// the various segments for program execution.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ProgramHeader {
    /// Program segment type
    pub progtype: ProgType,
    /// Offset into the ELF file where this segment begins
    pub offset: u64,
    /// Virtual adress where this segment should be loaded
    pub vaddr: u64,
    /// Physical address where this segment should be loaded
    pub paddr: u64,
    /// Size of this segment in the file
    pub filesz: u64,
    /// Size of this segment in memory
    pub memsz: u64,
    /// Flags for this segment
    pub flags: ProgFlag,
    /// file and memory alignment
    pub align: u64,
}

impl fmt::Display for ProgramHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program Header: Type: {} Offset: {:#010x} VirtAddr: {:#010x} PhysAddr: {:#010x} FileSize: {:#06x} MemSize: {:#06x} Flags: {} Align: {:#x}",
            self.progtype, self.offset, self.vaddr, self.paddr, self.filesz,
            self.memsz, self.flags, self.align)
    }
}

/// An ELF section type.
///
/// This is a field on [SectionHeader::shtype].
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum SectionType {
    /// Inactive section with undefined values. Named SHT_NULL in C code.
    Null = 0,
    /// Information defined by the program, includes executable code and data. Named SHT_PROGBITS in C code.
    Progbits = 1,
    /// Section data contains a symbol table. Named SHT_SYMTAB in C code.
    Symtab = 2,
    /// Section data contains a string table. Named SHT_SYMTAB in C code.
    Strtab = 3,
    /// Section data contains relocation entries with explicit addends. Named SHT_RELA in C code.
    Rela = 4,
    /// Section data contains a symbol hash table. Must be present for dynamic linking. Named SHT_HASH in C code.
    Hash = 5,
    /// Section data contains information for dynamic linking. Named SHT_DYNAMIC in C code.
    Dynamic = 6,
    /// Section data contains information that marks the file in some way. Named SHT_NOTE in C code.
    Note = 7,
    /// Section data occupies no space in the file but otherwise resembles SHT_PROGBITS. Named SHT_NOBITS in C code.
    Nobits = 8,
    /// Section data contains relocation entries without explicit addends. Named SHT_REL in C code.
    Rel = 9,
    /// Section is reserved but has unspecified semantics. Named SHT_SHLIB in C code.
    Shlib = 10,
    /// Section data contains a minimal set of dynamic linking symbols. Named SHT_DYNSYM in C code.
    Dynsym = 11,
    /// Section data contains an array of constructors. Named SHT_INIT_ARRAY in C code.
    InitArray = 12,
    /// Section data contains an array of destructors. Named SHT_FINI_ARRAY in C code.
    FiniArray = 13,
    /// Section data contains an array of pre-constructors. Named SHT_PREINIT_ARRAY in C code.
    PreinitArray = 14,
    /// Section group. Named SHT_GROUP in C code.
    Group = 15,
    /// Extended symbol table section index. Named SHT_SYMTAB_SHNDX in C code.
    SymtabShndx = 16,
    /// Number of reserved SHT_* values. Named SHT_NUM in C code.
    Num = 17,
    /// Object attributes. Named SHT_GNU_ATTRIBUTES in C code.
    GnuAttributes = 0x6fff_fff5,
    /// GNU-style hash section. Named SHT_GNU_HASH in C code.
    GnuHash = 0x6fff_fff6,
    /// Pre-link library list. Named SHT_GNU_LIBLIST in C code.
    GnuLiblist = 0x6fff_fff7,
    /// Version definition section. Named SHT_GNU_VERDEF in C code.
    GnuVerdef = 0x6fff_fffd,
    /// Version needs section. Named SHT_GNU_VERNEED in C code.
    GnuVerneed = 0x6fff_fffe,
    /// Version symbol table. Named SHT_GNU_VERSYM in C code.
    GnuVersym = 0x6fff_ffff,
    /// Arm specific. Named SHT_ARM_EXIDX in C code.
    ArmExidc = 0x7000_0001,
    /// Arm specific. Named SHT_ARM_ATTRIBUTES in C code.
    ArmAttributes = 0x7000_0003,
}

///
/// Wrapper type for SectionFlag
///
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SectionFlag(pub u64);
/// Empty flags
pub const SHF_NONE: SectionFlag = SectionFlag(0);
/// Writable
pub const SHF_WRITE: SectionFlag = SectionFlag(1);
/// Occupies memory during execution
pub const SHF_ALLOC: SectionFlag = SectionFlag(2);
/// Executable
pub const SHF_EXECINSTR: SectionFlag = SectionFlag(4);
/// Might be merged
pub const SHF_MERGE: SectionFlag = SectionFlag(16);
/// Contains nul-terminated strings
pub const SHF_STRINGS: SectionFlag = SectionFlag(32);
/// `sh_info' contains SHT index
pub const SHF_INFO_LINK: SectionFlag = SectionFlag(64);
/// Preserve order after combining
pub const SHF_LINK_ORDER: SectionFlag = SectionFlag(128);
/// Non-standard OS specific handling required
pub const SHF_OS_NONCONFORMING: SectionFlag = SectionFlag(256);
/// Section is member of a group
pub const SHF_GROUP: SectionFlag = SectionFlag(512);
/// Section hold thread-local data
pub const SHF_TLS: SectionFlag = SectionFlag(1024);

impl fmt::Debug for SectionFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Display for SectionFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

/// Encapsulates the contents of an ELF Section Header
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectionHeader {
    /// Section Name
    pub name: String,
    /// Section Type
    pub shtype: SectionType,
    /// Section Flags
    pub flags: SectionFlag,
    /// in-memory address where this section is loaded
    pub addr: u64,
    /// Byte-offset into the file where this section starts
    pub offset: u64,
    /// Section size in bytes
    pub size: u64,
    /// Defined by section type
    pub link: u32,
    /// Defined by section type
    pub info: u32,
    /// address alignment
    pub addralign: u64,
    /// size of an entry if section data is an array of entries
    pub entsize: u64,
}

impl fmt::Display for SectionHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Section Header: Name: {} Type: {} Flags: {} Addr: {:#010x} Offset: {:#06x} Size: {:#06x} Link: {} Info: {:#x} AddrAlign: {} EntSize: {}",
            self.name, self.shtype, self.flags, self.addr, self.offset,
            self.size, self.link, self.info, self.addralign, self.entsize)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SymbolType(pub u8);
/// Unspecified symbol type
pub const STT_NOTYPE: SymbolType = SymbolType(0);
/// Data object symbol
pub const STT_OBJECT: SymbolType = SymbolType(1);
/// Code object symbol
pub const STT_FUNC: SymbolType = SymbolType(2);
/// Section symbol
pub const STT_SECTION: SymbolType = SymbolType(3);
/// File name symbol
pub const STT_FILE: SymbolType = SymbolType(4);
/// Common data object symbol
pub const STT_COMMON: SymbolType = SymbolType(5);
/// Thread-local data object symbol
pub const STT_TLS: SymbolType = SymbolType(6);
/// Indirect code object symbol
pub const STT_GNU_IFUNC: SymbolType = SymbolType(10);

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            STT_NOTYPE => "unspecified",
            STT_OBJECT => "data object",
            STT_FUNC => "code object",
            STT_SECTION => "section",
            STT_FILE => "file name",
            STT_COMMON => "common data object",
            STT_TLS => "thread-local data object",
            STT_GNU_IFUNC => "indirect code object",
            _ => "Unknown",
        };
        write!(f, "{}", str)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SymbolBind(pub u8);
/// Local symbol
pub const STB_LOCAL: SymbolBind = SymbolBind(0);
/// Global symbol
pub const STB_GLOBAL: SymbolBind = SymbolBind(1);
/// Weak symbol
pub const STB_WEAK: SymbolBind = SymbolBind(2);
/// Unique symbol
pub const STB_GNU_UNIQUE: SymbolBind = SymbolBind(10);

impl fmt::Display for SymbolBind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            STB_LOCAL => "local",
            STB_GLOBAL => "global",
            STB_WEAK => "weak",
            STB_GNU_UNIQUE => "unique",
            _ => "Unknown",
        };
        write!(f, "{}", str)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SymbolVis(pub u8);
/// Default symbol visibility
pub const STV_DEFAULT: SymbolVis = SymbolVis(0);
/// Processor-specific hidden visibility
pub const STV_INTERNAL: SymbolVis = SymbolVis(1);
/// Hidden visibility
pub const STV_HIDDEN: SymbolVis = SymbolVis(2);
/// Protected visibility
pub const STV_PROTECTED: SymbolVis = SymbolVis(3);

impl fmt::Display for SymbolVis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            STV_DEFAULT => "default",
            STV_INTERNAL => "internal",
            STV_HIDDEN => "hidden",
            STV_PROTECTED => "protected",
            _ => "Unknown",
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Symbol value
    pub value: u64,
    /// Symbol size
    pub size: u64,
    /// Section index
    pub shndx: u16,
    /// Symbol type
    pub symtype: SymbolType,
    /// Symbol binding
    pub bind: SymbolBind,
    /// Symbol visibility
    pub vis: SymbolVis,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Symbol: Value: {:#010x} Size: {:#06x} Type: {} Bind: {} Vis: {} Section: {} Name: {}",
            self.value, self.size, self.symtype, self.bind, self.vis, self.shndx, self.name
        )
    }
}
