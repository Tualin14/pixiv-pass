use reqwest::header::{REFERER, USER_AGENT};
use worker::*;

use crate::crypto::decrypt;

mod crypto;

const PIXIV_REFERER: &str = "https://www.pixiv.net/";
const PIXIV_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

const PIXIV_META_URL: &str = "https://i.pximg.net";


fn set_file_type(decrypt_url: &str) -> &str {
    match decrypt_url
        .split('.')
        .last()
        .map(|ext| ext.to_lowercase())
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("bmp") => "image/bmp",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    }
}

fn get_file_name(decrypt_url: &str) -> &str {
    decrypt_url.split('/').last().unwrap_or_else(|| "download file")
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if let Ok(access_referer) = env.var("REQUEST_REFERER") {
        let request_headers = req.headers();
        let req_referer = match request_headers.get("Referer") {
            Ok(x) => match x {
                None => return Response::error("Bad Request", 400),
                Some(y) => y
            },
            Err(_) => return Response::error("Bad Request", 400),
        };
        if !req_referer.starts_with(access_referer.to_string().as_str()) {
            return Response::error("Bad Request", 400);
        }
    };

    Router::new()
        // GET.
        // cipherText empty.
        .get_async("/", |_req, _ctx| async move {
            Response::error("I'm a teapot", 418)
        })

        // GET.
        // cipherText.
        .get_async("/:cipherText", |_req, ctx| async move {
            if let Some(cipher_text) = ctx.param("cipherText") {
                let crypto_secret = match ctx.env.secret("CRYPTO_SECRET") {
                    Ok(x) => x.to_string(),
                    Err(_) => return Response::error("Internal Server Error: CRYPTO_SECRET not found", 500),
                };

                let decrypt_url = match decrypt(crypto_secret.as_str(), cipher_text.as_str()) {
                    Ok(x) => x,
                    Err(err) => return Response::error(err, 400)
                };

                if !decrypt_url.starts_with(PIXIV_META_URL) {
                    return Response::error("Bad Request", 400);
                }

                let client = reqwest::Client::new();
                let resp = match client
                    .get(decrypt_url.as_str())
                    .header(REFERER, PIXIV_REFERER)
                    .header(USER_AGENT, PIXIV_USER_AGENT)
                    .send().await {
                    Ok(x) => x,
                    Err(_err) => return Response::error("Bad Request", 400)
                };

                return if resp.status().is_success() {
                    let image_bytes = match resp.bytes().await {
                        Ok(x) => x.to_vec(),
                        Err(_) => return Response::error("Internal Server Error: Failed to make request to the remote server", 500),
                    };

                    let mut response = match Response::from_bytes(image_bytes) {
                        Ok(x) => x,
                        Err(_) => return Response::error("Internal Server Error: Failed to process image bytes", 500),
                    };

                    let response_headers = response.headers_mut();
                    if let Err(_) = response_headers.set("Content-Type", set_file_type(decrypt_url.as_str())) {
                        return Response::error("Internal Server Error: Failed to set Content-Type header", 500);
                    };
                    if let Err(_) = response_headers.set("Content-Disposition", format!("attachment; filename=\"{}\"", get_file_name(decrypt_url.as_str())).as_str()) {
                        return Response::error("Internal Server Error: Failed to set Content-Disposition header", 500);
                    }

                    Ok(response)
                } else {
                    match resp.text().await {
                        Ok(x) => Response::error(x, 400),
                        Err(_) => Response::error("Internal Server Error", 500),
                    }
                };
            }
            return Response::error("Bad Request", 400);
        })

//         // POST.
//         // multipart/form-data.
//         // fileName.
//         // cipherText.
//         .post_async("/", |mut req, ctx| async move {
//             let form = match req.form_data().await {
//                 Ok(x) => x,
//                 Err(_) => return Response::error("Bad Request", 400),
//             };
//
//             let cipher_text = match form.get("cipherText")
//             {
//                 None => return Response::error("Missing cipher text", 400),
//                 Some(entry) => {
//                     match entry {
//                         FormEntry::Field(x) => x,
//                         FormEntry::File(_) => return Response::error("Bad Request", 400),
//                     }
//                 }
//             };
//
//             let crypto_secret = match ctx.env.secret("CRYPTO_SECRET") {
//                 Ok(x) => x.to_string(),
//                 Err(_) => return Response::error("Internal Server Error: CRYPTO_SECRET not found", 500),
//             };
//
//             let decrypt_url = match decrypt(crypto_secret.as_str(), cipher_text.as_str()) {
//                 Ok(x) => x,
//                 Err(err) => return Response::error(err, 400)
//             };
//
//             let client = reqwest::Client::new();
//             let resp = match client
//                 .get(&decrypt_url)
//                 .header(REFERER, PIXIV_REFERER)
//                 .header(USER_AGENT, PIXIV_USER_AGENT)
//                 .send().await {
//                 Ok(x) => x,
//                 Err(_err) => return Response::error("Bad Request", 400)
//             };
//
//             return if resp.status().is_success() {
//                 let image_bytes = match resp.bytes().await {
//                     Ok(x) => x.to_vec(),
//                     Err(_) => return Response::error("Internal Server Error: Failed to make request to the remote server", 500),
//                 };
//
//                 let mut response = match Response::from_bytes(image_bytes) {
//                     Ok(x) => x,
//                     Err(_) => return Response::error("Internal Server Error: Failed to process image bytes", 500),
//                 };
//
//                 let response_headers = response.headers_mut();
//                 if let Err(_) = response_headers.set("Content-Type", set_file_type(decrypt_url.as_str())) {
//                     return Response::error("Internal Server Error: Failed to set Content-Type header", 500);
//                 };
//                 if let Err(_) = response_headers.set("Content-Disposition", format!("attachment; filename=\"{}\"", get_file_name(decrypt_url.as_str())).as_str()) {
//                     return Response::error("Internal Server Error: Failed to set Content-Disposition header", 500);
//                 }
//
//                 Ok(response)
//             } else {
//                 match resp.text().await {
//                     Ok(x) => Response::error(x, 400),
//                     Err(_) => Response::error("Internal Server Error", 500),
//                 }
//             };
//         })
        .run(req, env).await
}


