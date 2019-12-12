use os_info::{Info, Type};
use std::path::{Path, PathBuf};
use dirs::home_dir;
use crate::errors;

enum OsType {
    Darwin,
    Windows,
    Linux,
    Unsupported,
}

pub struct DirExt {
    pub dir: PathBuf,
    pub ext: String,
}

fn parse_os_type() -> OsType {
    let os_info = os_info::get();
    match os_info.os_type() {
        Type::Windows => OsType::Windows,
        Type::Macos => OsType::Darwin,
        Type::Android | Type::Emscripten => OsType::Unsupported,
        _ => OsType::Linux,
    }
}

pub fn get_app_dir() -> Result<DirExt, errors::PathError> {
    let os_type = parse_os_type();
    match os_type {
        OsType::Darwin => Ok(DirExt { dir: PathBuf::from("/Applicatiions"), ext: String::from("app") }),
        OsType::Windows => Ok(DirExt {
            dir: home_dir().ok_or_else(|| errors::PathError { kind: String::from("dirs"), message: String::from("Cannot acquire home dir.") })?.join("Start Menu"),
            ext: String::from("exe")
        }),
        OsType::Linux => Ok(DirExt { dir: PathBuf::from("/usr/share/applications"), ext: String::from("desktop") }),
        OsType::Unsupported => panic!("Unsupported OS"),
    }
}

mod tests {
    use super::*;

    
}
