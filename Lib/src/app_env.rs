/// The environment the app is running on. This includes the Operating System and other supported
/// environment variables
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AppEnvironment {
    /// The type of operating system the app is running on
    pub os: OsType,
    /// The operating system family the app is running on
    pub family: OsFamily,
    /// The cpu architecture the app is running on
    pub cpu_arch: CpuArchitecture,
    /// The extension used by shared libraries recognized by the current operating system the app is running on
    pub dyn_lib_ext: DynLibExtension,
    /// The prefix added to locate the shared libraries as recognized by the operating system the app is running on
    pub dyn_lib_prefix: DynLibPrefix,
}

impl AppEnvironment {
    /// Initialize the app's environment
    pub fn init() -> Self {
        AppEnvironment {
            os: OsType::which_os(),
            family: OsFamily::which_os_family(),
            cpu_arch: CpuArchitecture::which_cpu_arch(),
            dyn_lib_ext: DynLibExtension::which_dyn_lib_ext(),
            dyn_lib_prefix: DynLibPrefix::which_dyll_prefix(),
        }
    }
}

/// The type of operating system the Window is running on
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum OsType {
    /// Windows Operating System
    Windows,
    /// Apple's MacOS Operating System
    MacOs,
    /// Apple's iOS Operating System
    Ios,
    /// A Linux Kernel based Desktop Operating System
    Linux,
    /// Android Operating System
    Android,
    /// FreeBSD Operating System
    FreeBsd,
    /// Dragonfly BSD Operating System
    Dragonfly,
    /// OpenBSD Operating System
    OpenBsd,
    /// NetBSD Operating System
    NetBsd,
    /// Solaris Operating System
    Solaris,
    /// The Operating System is unrecognized
    UnrecognizedOs,
}

impl OsType {
    /// Detect the operating system the app is being run on
    pub fn which_os() -> Self {
        std::env::consts::OS.into()
    }
}

impl Into<OsType> for &str {
    fn into(self) -> OsType {
        match self {
            "linux" => OsType::Linux,
            "macos" => OsType::MacOs,
            "ios" => OsType::Ios,
            "freebsd" => OsType::FreeBsd,
            "dragonfly" => OsType::Dragonfly,
            "netbsd" => OsType::NetBsd,
            "openbsd" => OsType::OpenBsd,
            "solaris" => OsType::Solaris,
            "android" => OsType::Android,
            "windows" => OsType::Windows,
            "unrecognized_os" => OsType::UnrecognizedOs,
            _ => OsType::UnrecognizedOs,
        }
    }
}

/// Check if operating system is Unix-like or Microsoft Windows based
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum OsFamily {
    /// The operating system is `Unix-like`
    Unix,
    /// The operating system is `Microsoft Windows` based
    Windows,
    /// The family of the Operating System the app is running on is unrecognized
    UnrecognizedFamily,
}

impl OsFamily {
    /// Detect if the operating system is Unix-like
    pub fn which_os_family() -> Self {
        std::env::consts::FAMILY.into()
    }
}

impl Into<OsFamily> for &str {
    fn into(self) -> OsFamily {
        match self {
            "unix" => OsFamily::Unix,
            "windows" => OsFamily::Windows,
            "unrecognized_family" => OsFamily::UnrecognizedFamily,
            _ => OsFamily::UnrecognizedFamily,
        }
    }
}

/// What is the architecture of the CPU that the app is running on
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CpuArchitecture {
    /// Intel CPU architecture recognized as `x86` in Rust
    Intel32,
    /// Intel CPU architecture recognized as `x86_64` in Rust
    Intel64,
    /// Arm's 32 bit CPU architecture recognized as `arm` in Rust
    Arm32,
    /// Arm's 64 bit CPU architecture recognized as `aarch64` in Rust
    Arm64,
    /// China's Loongson Technology Corporation 64 bit CPU architecture recognized as `loongarch64` in Rust
    LoongArch64,
    /// MIPS Technologies 32 bit CPU architecture recognized as `mips` in Rust
    Mips32,
    /// MIPS Technologies 64 bit architecture recognized as `mips64` in Rust
    Mips64,
    /// Power 32 bit CPU architecture recognized as `powerpc` in Rust
    Powerpc32,
    /// Power 64 bit CPU architecture recognized as `powerpc64` in Rust
    Powerpc64,
    /// Risc-V 32 bit CPU architecture recognized as `riscv32` in Rust
    Riscv32,
    ///  Risc-V 64 bit CPU architecture recognized as `riscv64` in Rust
    Riscv64,
    /// An unsupported CPU architecture by this library and recognized as `unsupported_arch` .
    /// This does not necessarily mean that the architecture is unsupported by the Rust language.
    UnsupportedArch,
}

impl CpuArchitecture {
    /// Detect the operating system the app is being run on
    pub fn which_cpu_arch() -> Self {
        std::env::consts::ARCH.into()
    }
}

impl Into<CpuArchitecture> for &str {
    fn into(self) -> CpuArchitecture {
        match self {
            "x86" => CpuArchitecture::Intel32,
            "x86_64" => CpuArchitecture::Intel64,
            "arm" => CpuArchitecture::Arm32,
            "aarch64" => CpuArchitecture::Arm64,
            "loongarch64" => CpuArchitecture::LoongArch64,
            "mips" => CpuArchitecture::Mips32,
            "mips64" => CpuArchitecture::Mips64,
            "powerpc" => CpuArchitecture::Powerpc32,
            "powerpc64" => CpuArchitecture::Powerpc64,
            "riscv32" => CpuArchitecture::Riscv32,
            "riscv64" => CpuArchitecture::Riscv64,
            "unsupported_arch" => CpuArchitecture::UnsupportedArch,
            _ => CpuArchitecture::UnsupportedArch,
        }
    }
}

/// The extension used by the shared libraries on the Operating System the app is running on
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DynLibExtension {
    /// Mostly used in Linux operating system
    So,
    /// Used in Apple's operating systems
    Dylib,
    /// Used in Microsoft Windows operating systems
    Dll,
    /// The dynamic library extension is not recognized
    UnrecognizedDynLib,
}

impl DynLibExtension {
    /// Detect the operating system the app is being run on
    pub fn which_dyn_lib_ext() -> Self {
        std::env::consts::DLL_EXTENSION.into()
    }
}

impl Into<DynLibExtension> for &str {
    fn into(self) -> DynLibExtension {
        match self {
            "so" => DynLibExtension::So,
            "dylib" => DynLibExtension::Dylib,
            "dll" => DynLibExtension::Dll,
            "unrecognized_dyn_lib_extension" => DynLibExtension::UnrecognizedDynLib,
            _ => DynLibExtension::UnrecognizedDynLib,
        }
    }
}

/// The extension used by the shared libraries on the Operating System the app is running on.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DynLibPrefix {
    /// The `lib` prefix is used.
    Lib,
    /// The `""` prefix is used.
    None,
}

impl DynLibPrefix {
    /// Detect the operating system the app is being run on
    pub fn which_dyll_prefix() -> Self {
        std::env::consts::DLL_PREFIX.into()
    }
}

impl Into<DynLibPrefix> for &str {
    fn into(self) -> DynLibPrefix {
        match self {
            "lib" => DynLibPrefix::Lib,
            _ => DynLibPrefix::None,
        }
    }
}

#[cfg(test)]
mod sanity_test {
    use crate::{AppEnvironment, OsFamily, OsType};

    #[test]
    fn detect_os() {
        // Detect App OS
        if cfg!(target_os = "linux") {
            assert_eq!(OsType::Linux, OsType::which_os().into());
        } else if cfg!(target_os = "windows") {
            assert_eq!(OsType::Windows, OsType::which_os().into());
        } else if cfg!(target_os = "macos") {
            assert_eq!(OsType::MacOs, OsType::which_os().into());
        } else if cfg!(target_os = "silly_os") {
            assert_eq!(OsType::UnrecognizedOs, OsType::which_os().into());
        }
    }

    #[test]
    fn init_app_env() {
        // Detect app environment by initializing the `AppEnvironment` struct with `init()` method
        let init_app_env = AppEnvironment::init();

        if cfg!(target_os = "linux") {
            assert_eq!(init_app_env.os, OsType::which_os().into());
        } else if cfg!(target_os = "windows") {
            assert_eq!(init_app_env.os, OsType::which_os().into());
        } else if cfg!(target_os = "macos") {
            assert_eq!(init_app_env.os, OsType::which_os().into());
        } else {
        }

        if cfg!(target_arch = "unix") {
            assert_eq!(init_app_env.family, OsFamily::which_os_family().into());
        } else if cfg!(target_family = "windows") {
            assert_eq!(init_app_env.family, OsFamily::which_os_family().into());
        }
    }
}
