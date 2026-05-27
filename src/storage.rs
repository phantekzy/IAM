use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub fn data_dir() -> PathBuf {
    #[cfg(windows)]
    let base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    #[cfg(not(windows))]\n    let base = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".local").join("share"))
        .unwrap_or_else(|_| PathBuf::from("."));
    let d = base.join("IAMBusiness");
    std::fs::create_dir_all(&d).ok();
    d
}

