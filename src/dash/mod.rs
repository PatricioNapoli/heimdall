use std::sync::{Arc};
use std::{fs::File};
use std::io::prelude::*;
use std::collections::HashMap;

use hyper::{header, Body, Request, Method, Response};

use tera::Context;

use futures::{FutureExt};

use super::utils;
use super::error::Result;
use super::router;
use super::services;


pub fn get_routes() -> Vec<router::Route> {
    vec![
        router::Route::new("static", "/static/**"),
        router::Route::new("dash", ""),
        router::Route::new("dash", "/"),
        router::Route::new("login", "/login"),
    ]
}

pub async fn authenticate(req: &Request<Body>, svcs: &Arc<services::Services>) -> Option<String> {
    if !req.headers().contains_key("Cookies") {
        return None;
    }

    let token = utils::get_cookie(req.headers()["Cookies"].to_str().unwrap());
    if token == "" {
        return None;
    }

    svcs.session.clone().exists(token)
}

pub fn method_not_allowed() -> Result<Response<Body>> {
    Ok(Response::builder().status(405).body(Body::empty()).unwrap())
}

pub fn bad_request() -> Result<Response<Body>> {
    Ok(Response::builder().status(400).body(Body::empty()).unwrap())
}

pub fn not_found(req: Request<Body>, svcs: Arc<services::Services>) -> Result<Response<Body>> {
    let mut context = Context::new();
    context.insert("code", "404");
    context.insert("description", "We did not find that page.");

    let res = router::html_from_template("error/error.html", context);

    Ok(Response::new(res.into()))
}

pub fn serve_static(req: Request<Body>, svcs: Arc<services::Services>) -> Result<Response<Body>> {
    if req.method() != Method::GET {
        return method_not_allowed();
    }

    let mut path = String::from(req.uri().path());
    path = path.replace("/static", "./assets/public");

    let mime: mime::Mime;

    match utils::get_path_and_mime(path) {
        Ok(m) => {
            mime = m.mime;
            path = m.path;
        },
        Err(_) => {
            return not_found(req, svcs);
        }
    }

    let mut file = File::open(&path).unwrap();
    let len = file.metadata().unwrap().len() as usize;
    let mut data = Vec::new();
    data.resize(len, 0);
    file.read_exact(&mut data).unwrap();

    Ok(Response::builder()
        .header(header::CONTENT_LENGTH, len as u64)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(data.into())?
    )
}

async fn login(req: Request<Body>, svcs: Arc<services::Services>) -> Result<Response<Body>> {
    if req.method() != Method::POST {
        return login_response(req);
    }
    let entire_body = utils::get_body(req, 128);

    Ok(entire_body.map(|body| {
        if body.len() == 0 {
            return bad_request().unwrap();
        }

        let parsed: HashMap<_, String> = url::form_urlencoded::parse(body.as_slice()).into_owned().collect();
        if !parsed.contains_key("email") || !parsed.contains_key("password") {
            return bad_request().unwrap();
        }

        let email: String = String::from(parsed.get("email").unwrap());
        let body = Body::from(email);
        return Response::new(body);
    }).await)
}

pub fn login_response(req: Request<Body>) -> Result<Response<Body>> {
    let mut context = Context::new();
    context.insert("next", "");
    let res = router::html_from_template("landing/login.html", context);
    Ok(Response::new(res.into()))
}

pub async fn dash(req: Request<Body>, svcs: Arc<services::Services>) -> Result<Response<Body>> {
    if req.method() != Method::GET {
        return method_not_allowed();
    }

    return match authenticate(&req, &svcs).await {
        Some(user_id) => {
            let res = router::html_from_template("dash/hq.html", Context::new());
            Ok(Response::builder().body(res.into()).unwrap())
        },
        None => login_response(req)
    }
}

pub async fn handle(req: Request<Body>, svcs: Arc<services::Services>) -> Result<Response<Body>> {
    let path = req.uri().path();
    let router = svcs.router.clone();

    return match &router.route(path) {
        Ok(name) => {
            match *name {
                "static" => serve_static(req, svcs),
                "dash" => dash(req, svcs).await,
                "login" => login(req, svcs).await,
                _ => not_found(req, svcs)
            }
        }
        Err(_) => not_found(req, svcs)
    }
}
