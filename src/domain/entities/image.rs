use delphinium::entities::file::File;

#[derive(Debug, Clone)]
pub struct UpstreamImage {
    pub upstream_hash: String,
    pub file_name: String,
    pub width: i32,
    pub height: i32,
}
#[derive(Debug, Clone)]
pub struct Image {
    pub file_name: String,
    pub hash: String,
    pub upstream: File,
}

impl Image {
    pub fn new(file_name: String, hash: String, file: File) -> Self {
        Image {
            file_name,
            hash,
            upstream: file,
        }
    }
}
