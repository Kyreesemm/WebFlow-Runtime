use rust_embed::RustEmbed;
use std::borrow::Cow;
use wry::http::{HeaderValue, Response};

#[derive(RustEmbed)]
#[folder = "webui/"]
pub struct WebUIAssets;

fn material_asset(path: &str) -> Option<(&'static [u8], &'static str)> {
    match path {
        "materials/default/webflow_runtime_icon.png" => Some((
            include_bytes!("../../materials/default/webflow_runtime_icon.png"),
            "image/png",
        )),
        "materials/fonts/Roboto-Regular.ttf" => Some((
            include_bytes!("../../materials/fonts/Roboto-Regular.ttf"),
            "font/ttf",
        )),
        "materials/fonts/Roboto-Medium.ttf" => Some((
            include_bytes!("../../materials/fonts/Roboto-Medium.ttf"),
            "font/ttf",
        )),
        "materials/fonts/Roboto-Bold.ttf" => Some((
            include_bytes!("../../materials/fonts/Roboto-Bold.ttf"),
            "font/ttf",
        )),
        "materials/fonts/MaterialSymbolsRounded.ttf" => Some((
            include_bytes!("../../materials/fonts/MaterialSymbolsRounded.ttf"),
            "font/ttf",
        )),
        _ => None,
    }
}

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

    if let Some((data, mime)) = material_asset(file_path) {
        return Response::builder()
            .status(200)
            .header("Content-Type", mime)
            .body(Cow::Borrowed(data))
            .unwrap();
    }

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

#[cfg(test)]
mod tests {
    use super::handle_custom_protocol_request;
    use wry::http::StatusCode;

    #[test]
    fn serves_index_for_supported_root_urls() {
        for path in ["/", "webflow://manager/", "http://webflow.localhost/"] {
            let response = handle_custom_protocol_request(path);
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers().get("Content-Type").and_then(|v| v.to_str().ok()), Some("text/html"));
            assert!(!response.body().is_empty());
        }
    }

    #[test]
    fn serves_bundled_fonts_with_correct_mime_type() {
        let response = handle_custom_protocol_request("materials/fonts/Roboto-Regular.ttf");
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("Content-Type").and_then(|v| v.to_str().ok()), Some("font/ttf"));
        assert!(!response.body().is_empty());
    }

    #[test]
    fn returns_not_found_for_unknown_assets() {
        let response = handle_custom_protocol_request("missing-asset.txt");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(response.body().as_ref(), b"404 Not Found");
    }
}
