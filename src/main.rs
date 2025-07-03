use std::{
    fs::read,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Component, Path, PathBuf},
    time::Duration,
};

fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1";
    const PORT: &str = "4242";
    let listener = TcpListener::bind(format!("{ADDRESS}:{PORT}"))?;
    println!("Server running on http://{ADDRESS}:{PORT}");

    for stream in listener.incoming().flatten() {
        handle_client(stream);
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(5))) {
        eprintln!("Failed to set read timeout: {e}");
        return;
    }
    if let Err(e) = stream.set_write_timeout(Some(Duration::from_secs(5))) {
        eprintln!("Failed to set write timeout: {e}");
        return;
    }

    if let Ok(peer_addr) = stream.peer_addr() {
        println!("Request from {peer_addr}");
    }

    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();

    if reader.read_line(&mut request_line).is_err() {
        let _ = respond_400(&mut stream);
        return;
    }

    println!("Request line: {}", request_line.trim());

    let mut parts = request_line.split_whitespace();
    let method = parts.next();
    let path = parts.next();
    let http_version = parts.next();

    if method != Some("GET") || http_version != Some("HTTP/1.1") || path.is_none() {
        let _ = respond_400(&mut stream);
        return;
    }

    for line in reader.by_ref().lines().map_while(Result::ok).take(100) {
        if line.len() > 8192 {
            let _ = respond_400(&mut stream);
            return;
        }
        if line.trim().is_empty() {
            break;
        }
    }

    let path = sanitize_path(path.unwrap());

    if !is_extension_allowed(&path) {
        let _ = respond_400(&mut stream);
        return;
    }

    const MAX_CONTENT_LENGTH: usize = 10 * 1024 * 1024; // 10 MB

    match read(&path) {
        Ok(contents) if contents.len() <= MAX_CONTENT_LENGTH => {
            let content_type = guess_content_type(&path);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                contents.len(),
                content_type
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(&contents);
        }
        Ok(_) => {
            let _ =
                stream.write_all(b"HTTP/1.1 413 Payload Too Large\r\n\r\n413 - Payload Too Large");
        }
        Err(_) => {
            let _ = respond_404(&mut stream);
        }
    }
}

fn sanitize_path(request_path: &str) -> PathBuf {
    let mut base = PathBuf::from("site");
    let clean_path = request_path.trim_start_matches('/');

    let mut safe_path = PathBuf::new();
    for component in PathBuf::from(clean_path).components() {
        match component {
            Component::Normal(comp) => safe_path.push(comp),
            _ => continue,
        }
    }

    if safe_path.as_os_str().is_empty() {
        base.push("index.html");
    } else {
        base.push(safe_path);
    }

    base
}

fn respond_400(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n400 - Bad Request")
}

fn respond_404(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n404 - File Not Found")
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

fn is_extension_allowed(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("html" | "css" | "js" | "png" | "jpg" | "jpeg" | "gif" | "svg")
    )
}
