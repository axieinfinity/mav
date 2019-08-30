#[macro_export]
macro_rules! file_stem {
    () => {
        std::path::Path::new(file!())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    };
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Os {
    MacOs,
    Linux,
    Windows,
    Other,
}

#[cfg(target_os = "macos")]
pub const OS: Os = Os::MacOs;
#[cfg(target_os = "linux")]
pub const OS: Os = Os::Linux;
#[cfg(target_os = "windows")]
pub const OS: Os = Os::Windows;
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub const OS: Os = Os::Other;
