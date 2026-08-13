use rust_embed::RustEmbed;
use std::borrow::Cow;
use wry::http::{HeaderValue, Response};

#[derive(RustEmbed)]
#[folder = "webui/"]
pub struct WebUIAssets;

pub fn handle_custom_protocol_request(path: &str) -> Response<Cow<'static, [u8]>> {
    let clean_path = path
        .trim_start_matches('/')
        .trim_start_matches("webflow://manager")
        .trim_start_matches("http://webflow.localhost");

    let file_path = if clean_path.is_empty() || clean_path == "/" {
        "index.html"
    } else {
        clean_path.trim_start_matches('/')
    };

    if let Some(asset) = WebUIAssets::get(file_path) {
        let mime = mime_guess::from_path(file_path)
            .first_or_text_plain()
            .to_string();

        let mut builder = Response::builder()
            .status(200)
            .header("Content-Type", mime);

        let val = HeaderValue::from_static("cross-origin");
        builder = builder.header("Cross-Origin-Embedder-Policy", val);

        builder
            .body(Cow::Owned(asset.data.into_owned()))
            .unwrap()
    } else {
        Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(Cow::Borrowed(&b"404 Not Found"[..]))
            .unwrap()
    }
}
