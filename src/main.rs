use byteorder::{LittleEndian, ReadBytesExt};
use delphinium::Delphinium;
use hyacinth::domain::entities::image::Image;
use hyacinth::domain::entities::metadata::Metadata;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use sha2::{Digest, Sha256};
use std::env;
use std::io::Cursor;
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "https://heliotrope.saebasol.org";

    let client = Client::builder()
        .user_agent(format!("Hyacinth/{}", env!("CARGO_PKG_VERSION")))
        .build()?;
    let delphinium = Delphinium::new(BASE_URL.to_string(), client.clone());
    let binary = client
        .get(format!("{}/api/hitomi/id", BASE_URL))
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    let mut cursor = Cursor::new(binary);
    let mut ids = Vec::new();
    while let Ok(id) = cursor.read_u32::<LittleEndian>() {
        ids.push(id);
    }

    println!("Received {:?} ids", ids);

    for id in ids {
        let info = delphinium.get_info(id as i32).await.unwrap();
        let images = delphinium.get_image(id as i32).await.unwrap();

        let mut image_vec: Vec<Image> = vec![];
        let dir_path = format!("images/{}", id);
        fs::create_dir_all(&dir_path).await?;

        for (index, img) in images.iter().enumerate() {
            let url = format!(
                "{}/api/proxy/{}",
                BASE_URL,
                utf8_percent_encode(&img.url, NON_ALPHANUMERIC)
            );

            let resp = client.get(&url).send().await?.error_for_status()?;

            let content_type = resp
                .headers()
                .get(CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let bytes = resp.bytes().await?;

            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            let hash = hasher.finalize();
            let hash = format!("{:x}", hash);

            let file_path = Path::new(&dir_path).join(format!(
                "{}.{}",
                index,
                content_type.split('/').nth(1).unwrap()
            ));

            let image = Image::new(
                file_path.file_name().unwrap().to_str().unwrap().to_string(),
                hash,
                img.file.clone(),
            );
            image_vec.push(image);
            fs::write(file_path, &bytes).await?;
        }

        let metadata = Metadata::new(id as i32, info, image_vec, chrono::Utc::now().naive_utc());
        println!("Saved metadata for {:?}", metadata);
    }
    Ok(())
}
