use std::sync::{Arc};
use hyper::{Body, Request, Response, StatusCode};

use super::error::Result;
use super::services;
use super::cache;
use super::security;
use super::dash;

fn bad_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::default()).unwrap()
}

fn unauthorized() -> Response<Body> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::default()).unwrap()
}

fn token_auth(req: &Request<Body>, svc: &cache::Route, svcs: &Arc<services::Services>) -> bool {
    if svc.auth != "1" {
        return true;
    }

    if !req.headers().contains_key("Authentication") {
        return false;
    }

    let auth_header = req.headers()["Authentication"].to_str().unwrap();

    if auth_header.len() >= 64 {
        return false;
    }

    let auth_pair = auth_header.replace("Basic ", "");
    let auth_split: Vec<&str> = auth_pair.split(" ").collect();
    let user = auth_split[0];
    let pass = auth_split[1];

    security::verify_hash(pass, pass, &svcs.config.clone())
}

pub async fn handle(req: Request<Body>, svcs: Arc<services::Services>) -> Result<Response<Body>> {
    if req.headers().contains_key("Host") {
        let host = req.headers()["Host"].to_str().unwrap();
        let config = svcs.config.clone();

        if host == &config.heimdall_hq {
            return dash::handle(req, svcs).await;
        }

        if svcs.data.contains_key(host) {
            let svc = svcs.data.get(host).unwrap();

            async fn proxy() -> Result<Response<Body>> {
                Ok(Response::new("hi".into()))
            }

            if svc.auth == "1" {
                let authenticated = token_auth(&req, &svc, &svcs);
                if authenticated {
                    return proxy().await;
                }

                return match dash::authenticate(&req, &svcs).await {
                    Some(user_id) => proxy().await,
                    None => Ok(unauthorized())
                }
            }

            return proxy().await;
        }

        return dash::not_found(req, svcs);
    }

    Ok(bad_request())
}
