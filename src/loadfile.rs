use std::fs;

#[derive(Debug, Clone, Default)]
pub struct ImageInfo {
    pub path: String,
    pub name: String,
    // pub width: u32,
    // pub height: u32,
}
pub async fn load() -> Vec<ImageInfo> {
    let mut list = vec![];
    let paths = rfd::AsyncFileDialog::new().pick_folder().await.unwrap();

    for path in walkdir::WalkDir::new(paths.path().display().to_string())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| is_image_file(&e.file_name().to_str().unwrap()))
    {
        let file_name = path.file_name().to_str().unwrap().to_string();
        let file_path = path.path().display().to_string();
        
        list.push(ImageInfo {
            path: file_path,
            name: file_name,
            // width: img.width(),
            // height: img.height(),
        });
    }
    list
}

fn is_image_file(f: &str) -> bool {
    let images_exts: Vec<&str> = vec!["png", "jpeg", "webp", "pnm", "ico", "avif"];
    for x in &images_exts {
        if f.ends_with(x) {
            return true;
        }
    }
    return false;
}
