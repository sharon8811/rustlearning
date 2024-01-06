use actix_files as fs;
use actix_web::Result;

pub async fn send_fav_icon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/favicon.ico")?)
}

