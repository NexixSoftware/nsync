use std::{io::Error, str::Utf8Error};

use base64::{Engine, prelude::BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{SEM_VER, storage::{backend::StorageBackend, vendor::r2}};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    version: String,
    file_name: String,
    contents_hash: String,
    encryption_alg: String,
    hashing_alg: String,
}

pub fn format_manifest(
    file_name: String,
    alg: String,
    contents_hash: [u8; 32],
) -> serde_json::Value {
    let m = Manifest {
        version: String::from(SEM_VER),
        file_name,
        contents_hash: BASE64_STANDARD.encode(contents_hash),
        encryption_alg: alg,
        hashing_alg: String::from("BLAKE3"),
    };

    json!(m)
}

pub fn write_manifest(file_name: String, alg: String, contents_hash: [u8; 32]) {
    let manifest = format_manifest(file_name, alg, contents_hash);
}
