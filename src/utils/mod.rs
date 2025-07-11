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
