// src/storage/vendor/r2.rs
use s3::{self, Bucket, Region};
use std::{os::unix::fs::MetadataExt, path::Path};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, BufReader},
};

use crate::{
    config::credentials::{AllowedCredentials, get_credentials},
    storage::backend::StorageBackend,
};

pub struct R2Controller {
    account_id: String,
    bucket_name: &'static str,
    credentials: AllowedCredentials,
}

const PART_SIZE: usize = 6 * 1024 * 1024; // 6 MiB

impl StorageBackend for R2Controller {
    // TODO: pipe in encryption logic when Cipher is complete
    // upload will upload a file thats >6mib
    async fn upload(&self, file_path: &Path) -> Option<[u8; 32]> {
        let file_metadata = fs::metadata(file_path).await.unwrap();
        if file_metadata.size() < PART_SIZE as u64 {
            return None;
        }

        let bucket_name = self.bucket_name;
        let region = Region::R2Eu {
            account_id: self.account_id.clone(),
        };

        let credentials = get_credentials(&self.credentials);

        let bucket = Bucket::new(bucket_name, region, credentials)
            .unwrap()
            .with_path_style();

        let file_type = file_metadata.file_type();
        let file_type = format!("{:?}", file_type);
        let file_name = file_path
            .file_name()
            .expect("failed to get file_name")
            .to_str()
            .expect("failed to transpose to str");

        let mut file = BufReader::new(File::open(file_path).await.expect("failed to open file"));

        let upload = bucket
            .initiate_multipart_upload(file_name, &file_type)
            .await
            .expect("failed to initiate multipart upload");

        let mut blake3 = blake3::Hasher::new();

        let mut parts = vec![];
        let mut part_number: u32 = 1;
        let mut running_total = 0;

        loop {
            let mut buf = vec![0u8; PART_SIZE];
            while running_total < PART_SIZE {
                let n = match file.read(&mut buf[running_total..]).await {
                    Ok(n) => n,
                    Err(e) => {
                        bucket.abort_upload(file_name, &upload.upload_id).await.ok();
                        panic!("failed to read from file: {e:?}");
                    }
                };

                if n == 0 {
                    break; // EOF
                }

                running_total += n;
            }

            if running_total == 0 {
                break; // EOF
            }
            buf.truncate(running_total);

            // update hash with buffer content
            blake3.update(&buf);

            match bucket
                .put_multipart_chunk(buf, file_name, part_number, &upload.upload_id, &file_type)
                .await
            {
                Ok(part) => parts.push(part),
                Err(e) => {
                    bucket.abort_upload(file_name, &upload.upload_id).await.ok();
                    panic!("multipart upload failed: {e}");
                }
            }

            part_number += 1;
            running_total = 0;
        }

        bucket
            .complete_multipart_upload(file_name, &upload.upload_id, parts)
            .await
            .expect("failed to complete multipart upload");

        Some(blake3.finalize().as_bytes().clone())
    }

    // Small file uploads <5mib
    async fn put(&self, file_path: &Path) {
        todo!()
    }

    //
    async fn put_manifest(&self, m: serde_json::Value) {
        todo!()
    }

    async fn delete() {
        todo!()
    }

    async fn query() {
        todo!()
    }

    async fn list() {
        todo!()
    }
}
