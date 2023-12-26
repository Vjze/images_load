
#[derive(Debug, Clone, Default)]
pub struct ImageInfo {
    pub path: String,
    pub name: String,
}

pub async fn load() -> Result<Vec<ImageInfo>,Error> {
    let mut list = vec![];
    let paths = rfd::AsyncFileDialog::new().pick_folder().await.ok_or(Error::DialogClosed)?;
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
        });
    }
    
    if list.is_empty(){
        Err(Error::ListNone)
        
    }else{
        Ok(list)
    }
    
}

fn is_image_file(f: &str) -> bool {
    let images_exts: Vec<&str> = vec![".png", ".jpeg", ".webp", ".pnm", ".ico", ".avif", ".jpg", ".gif", ".JPG", ".GIF", ".PNG", ".JPRG", ".WEBP", ".PNM", ".ICO", ".AVIF"];
    for x in &images_exts {
        if f.ends_with(x) {
            return true;
        }
    }
    return false;
}
#[derive(Debug, Clone, Default)]
pub enum Error {
    DialogClosed,
    #[default]
    ListNone,
}