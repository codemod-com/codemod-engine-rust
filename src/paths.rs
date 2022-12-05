use std::{collections::hash_map::DefaultHasher, ffi::OsStr, hash::Hasher, path::PathBuf};

fn build_new_path_buf(old_path_buf: &PathBuf) -> PathBuf {
    let file_stem = old_path_buf.file_stem().unwrap_or_default();

    let mut new_path_buf: PathBuf = old_path_buf
        .into_iter()
        .map(|osstr| {
            if osstr == "pages" {
                return OsStr::new("app");
            }

            return osstr;
        })
        .collect();

    new_path_buf.pop();

    if file_stem != "index" {
        new_path_buf.push(file_stem);
    }

    new_path_buf
}

pub fn get_pages_path_buf(old_path_buf: &PathBuf) -> Option<PathBuf> {
    let mut new_path_buf = PathBuf::new();

    for osstr in old_path_buf {
        new_path_buf.push(osstr);

        if osstr == "pages" {
            return Some(new_path_buf);
        }
    }

    return None
}

pub struct PathDto {
    pub old_path: String,
    pub new_page_path: String,
    pub new_head_path: String,
    pub page_output_path: String,
    pub head_output_path: String,
}

pub fn build_byte_hash(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(bytes);
    return hasher.finish();
}

pub fn build_output_path(
    output_directory_path: &String,
    new_page_path: &String,
    extension: &str,
) -> String {
    let hash = build_byte_hash(new_page_path.as_bytes());

    let file_name = format!("{:x}.{}", hash, extension);

    let output_path_buf: PathBuf = [output_directory_path, &file_name].iter().collect();

    output_path_buf.to_str().unwrap().to_string()
}

pub fn build_path_dto(output_directory_path: &String, old_path_buf: &PathBuf) -> Option<PathDto> {
    let extension = old_path_buf
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap();

    if old_path_buf.ends_with(String::from("_document.") + extension) {
        return None
    }

    if old_path_buf.ends_with(String::from("_app.") + extension) {
        return None
    }

    let new_path_buf = build_new_path_buf(&old_path_buf);

    let mut new_page_path_buf = new_path_buf.clone();
    new_page_path_buf.push(String::from("page.") + extension);

    let mut new_head_path_buf = new_path_buf.clone();
    new_head_path_buf.push(String::from("head.") + extension);

    let old_path = old_path_buf.to_str().unwrap().to_string();

    let new_page_path = new_page_path_buf.to_str().unwrap().to_string();
    let new_head_path = new_head_path_buf.to_str().unwrap().to_string();

    let page_output_path = build_output_path(output_directory_path, &new_page_path, extension);

    let head_output_path = build_output_path(output_directory_path, &new_head_path, extension);

    let path_dto = PathDto {
        old_path,
        new_page_path,
        new_head_path,
        page_output_path,
        head_output_path,
    };

    Some(path_dto)
}

pub fn build_page_document_path_buf_option(
    pages_path_buf: &PathBuf,
) -> Option<PathBuf> {
    let file_names = ["_document.js", "_document.jsx", "_document.ts", "_document.tsx"];

    let path_bufs: Vec<PathBuf> = file_names
        .iter()
        .map(|s| {
            let mut path_buf = pages_path_buf.clone();
            path_buf.push(s);

            path_buf
        })
        .filter(|path_buf| path_buf.exists())
        .collect();

    path_bufs.first().map(|p| p.to_owned())
}
