#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
pub const SONIC_ANNOTATOR_BINARY: &str = "win64/sonic-annotator-win64.exe";
#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
pub const VAMP_PLUGINS_PATH: &str = "win64/";

#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
pub const SONIC_ANNOTATOR_BINARY: &str = "win32/sonic-annotator-win32.exe";
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
pub const VAMP_PLUGINS_PATH: &str = "win32/";

#[cfg(all(target_os = "macos"))]
pub const SONIC_ANNOTATOR_BINARY: &str = "sonic-annotator-macos";
#[cfg(all(target_os = "macos"))]
pub const VAMP_PLUGINS_PATH: &str = "macos/";

#[cfg(all(target_os = "linux"))]
pub const SONIC_ANNOTATOR_BINARY: &str = "sonic-annotator-linux";
#[cfg(all(target_os = "linux"))]
pub const SONIC_ANNOTATOR_BINARY: &str = "linux/";