use std::{path::PathBuf, ffi::OsStr};

fn build_new_path_buf (
    old_path_buf: &PathBuf,
) -> PathBuf {
    let file_stem = old_path_buf.file_stem().unwrap_or_default();

    let mut new_path_buf: PathBuf = old_path_buf.into_iter().map(|osstr| {
        if osstr == "pages" {
            return OsStr::new("apps")
        }

        return osstr;
    }).collect();

    new_path_buf.pop();

    if file_stem != "index" {
        new_path_buf.push(file_stem);
    }

    new_path_buf
}

pub struct PathDto {
    pub old_path: String,
    pub new_dir_path: String,
    pub new_page_path: String,
    pub new_head_path: String,
}

pub fn build_path_dto (
    old_path_buf: PathBuf,
) -> PathDto {
    let extension = old_path_buf.extension().unwrap_or_default().to_str().unwrap();
    
    let new_path_buf = build_new_path_buf(&old_path_buf);

    let new_dir_path_buf = new_path_buf.clone();
    let mut new_page_path_buf = new_path_buf.clone();
    let mut new_head_path_buf = new_path_buf.clone();
   
    {
        let name = "page.".to_owned() + extension;

        new_page_path_buf.push(name);
    }

    {
        let name = "head.".to_owned() + extension;

        new_head_path_buf.push(name);
    }

    let old_path = old_path_buf.to_str().unwrap().to_string();

    let new_dir_path = new_dir_path_buf.to_str().unwrap().to_string();
    let new_page_path = new_page_path_buf.to_str().unwrap().to_string();
    let new_head_path = new_head_path_buf.to_str().unwrap().to_string();

    return PathDto {
        old_path,
        new_dir_path,
        new_page_path,
        new_head_path,
    }
    
}    

    