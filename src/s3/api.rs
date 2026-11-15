use std::{fs::File, path::Path};

use s3::{self, Bucket, Region, creds::Credentials};

pub struct API {}

pub async fn test() {
    let bucket_name = "nsync-dev";
    let region = Region::R2Eu {
        account_id: "d452f644c1f4b9d931d76d1d10e33089".to_string(),
    };

    let access_key = Some("eb174414851b7f2687df6d79d5dd5732");
    let secret_key = Some("b28660a8f6a68661ddf67458b2e6f0d3e2be9f1e67701aa098e30324891dd09a");

    let credentials = Credentials::new(access_key, secret_key, None, None, None)
        .expect("failed to parse from env");

    let bucket = Bucket::new(bucket_name, region, credentials)
        .unwrap()
        .with_path_style();

    let file = File::open(Path::new(""));

    todo!("open the file & read into a buffer to upload to r2");

    let upload = bucket
        .initiate_multipart_upload("", "application/text")
        .await
        .unwrap();

    bucket.put_multipart_chunk(chunk, path, part_number, upload_id, content_type)
}
