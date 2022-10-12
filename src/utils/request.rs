use std::collections::HashMap;

use hyper::{Body, Request};

use futures::{TryStreamExt, StreamExt};

pub struct PathMime {
    pub mime: mime::Mime,
    pub path: String
}

pub fn get_cookie(_full: &str) -> String {
    if _full.len() == 0 {
        return String::from("");
    }

    let parsed: HashMap<_, _> = url::form_urlencoded::parse(_full.as_bytes()).into_owned().collect();
    if parsed.contains_key("token_id") {
        return String::from(parsed.get("token_id").unwrap());
    }

    return String::from("");
}

pub fn get_mime(ext: &str) -> mime::Mime {
    mime_guess::from_ext(ext).first_or_octet_stream()
}

pub fn get_path_and_mime(path: String) -> std::result::Result<PathMime, &'static str> {
    if path.contains("..") {
        return Err("Contains backreference.")
    }

    let query_end = path.find('?').unwrap_or(path.len());
    let req_path = &path[0..query_end];

    let ext_pos = req_path.rfind(".").unwrap() + 1;
    let ext = &req_path[ext_pos..req_path.len()];

    let mime = mime_guess::from_ext(ext).first_or_octet_stream();

    return Ok(
        PathMime {
            mime,
            path: String::from(req_path)
        }
    );
}

pub async fn get_body(req: Request<Body>, max_size: usize) -> Vec<u8> {
    let (head, body) = req.into_parts();

    if !head.headers.contains_key("Content-Length") {
        return Vec::new();
    }

    let len_str = head.headers["Content-Length"].to_str().unwrap();
    let len: usize;
    match len_str.parse::<usize>() {
        Ok(l) => len = l,
        Err(e) => return Vec::new()
    }

    if len > max_size {
        return Vec::new();
    }

    let entire_body = body.take(max_size).try_fold(Vec::new(), |mut data, chunk| async move {
        data.extend_from_slice(&chunk);
        Ok(data)
    }).await;

    return entire_body.unwrap();
}
