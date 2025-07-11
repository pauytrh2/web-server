use std::{io::Write, net::TcpStream, path::Path};

pub fn respond_400(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n400 - Bad Request")
}

pub fn respond_404(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n404 - File Not Found")
}

pub fn is_extension_allowed(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("html" | "css" | "js" | "png" | "jpg" | "jpeg" | "gif" | "svg")
    )
}

fn guess_content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}
