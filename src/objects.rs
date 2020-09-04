
use std::env;
use bytes::{Bytes};
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};
//use async_trait::async_trait;

use crate::time_provider::SystemTimeProvider;
use crate::dynamodb::DynamodbStorage;



#[derive(Debug, Clone)]
pub struct OnetimeDownloaderConfig {
    pub api_key_files: String,
    pub api_key_links: String,
    pub max_len_file: usize,
    pub max_len_value: usize,
}

impl OnetimeDownloaderConfig {
    const EMPTY_STRING: String = String::new();
    const DEFAULT_MAX_LEN_FILE: usize = 100000;
    const DEFAULT_MAX_LEN_VALUE: usize = 80;

    pub fn env_var_string (name: &str, default: String) -> String {
        env::var(name).unwrap_or(default)
    }

    fn env_var_parse<T : std::str::FromStr> (name: &str, default: T) -> T {
        match env::var(name) {
            Ok(s) => s.parse::<T>().unwrap_or(default),
            _ => default
        }
    }

    pub fn from_env () -> OnetimeDownloaderConfig {
        OnetimeDownloaderConfig {
            api_key_files: Self::env_var_string("FILES_API_KEY", Self::EMPTY_STRING),
            api_key_links: Self::env_var_string("LINKS_API_KEY", Self::EMPTY_STRING),
            max_len_file: Self::env_var_parse("FILE_MAX_LEN", Self::DEFAULT_MAX_LEN_FILE),
            max_len_value: Self::env_var_parse("VALUE_MAX_LEN", Self::DEFAULT_MAX_LEN_VALUE),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OnetimeFile {
    pub filename: String,
    pub contents: Bytes,
    pub created_at: u64,
    pub updated_at: u64,
}

// https://serde.rs/impl-serialize.html
impl Serialize for OnetimeFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OnetimeFile", 4)?;
        state.serialize_field("filename", &self.filename)?;
        // only size of contents because we don't want to send entire files back... (and no default serializer for bytes)
        state.serialize_field("contents_len", &self.contents.len())?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("updated_at", &self.updated_at)?;
        state.end()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OnetimeLink {
    pub token: String,
    pub filename: String,
    pub created_at: u64,
    pub downloaded_at: Option<u64>,
    pub ip_address: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateLink {
    pub filename: String,
}

// #[async_trait]
// trait OnetimeStorage {
//     async fn add_file (&self, filename: String, contents: Bytes) -> Result<bool, String>;
//     async fn list_files (&self) -> Result<Vec<OnetimeFile>, String>;
//     async fn get_file (&self, filename: String) -> Result<OnetimeFile, String>;
//     async fn add_link (&self, link: String, filename: String) -> Result<bool, String>;
//     async fn list_links (&self) -> Result<Vec<OnetimeLink>, String>;
//     async fn get_link (&self, token: String) -> Result<OnetimeLink, String>;
// }

#[derive(Clone)]
pub struct OnetimeDownloaderService {
    pub time_provider: SystemTimeProvider,
    pub config: OnetimeDownloaderConfig,
    pub storage: DynamodbStorage,
}
