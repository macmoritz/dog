#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub default: _Config__default,
    pub features: _Config__features,
    pub only_use_config: _Config__only_use_config,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__default {
    pub buffer: i64,
    pub list_dir: bool,
    pub list_dir_depth: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__features {
    pub line_numbers: bool,
    pub show_ends: bool,
    pub show_statistics: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__only_use_config {
    pub mode: bool,
    pub warning: bool,
}

pub const CONFIG: Config = Config {
    default: _Config__default {
        buffer: 50,
        list_dir: false,
        list_dir_depth: 1,
    },
    features: _Config__features {
        line_numbers: true,
        show_ends: false,
        show_statistics: true,
    },
    only_use_config: _Config__only_use_config {
        mode: true,
        warning: false,
    },
};

#[cfg(debug_assertions)]
impl Config {
    pub fn load() -> Cow<'static, Self> {
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "//home/moritz/.config/dog/config.yaml");
        Self::load_from(filepath.as_ref()).expect("Failed to load Config.")
    }

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = ::serde_yaml::from_str(&file_contents)?;
        Ok(Cow::Owned(result))
    }
}

#[cfg(not(debug_assertions))]
impl Config {
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {
        Cow::Borrowed(&CONFIG)
    }

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        Ok(Cow::Borrowed(&CONFIG))
    }
}
