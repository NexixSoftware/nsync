use std::path::Path;

pub trait StorageBackend {
    async fn upload(&self, file_path: &Path) -> [u8; 32]; // multipart

    async fn put(); // small files

    async fn delete();

    async fn query();

    async fn list();
}
