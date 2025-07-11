use {
    std::{
        fs::read,
        io::{BufRead, BufReader, Read, Write},
        net::{TcpListener, TcpStream},
        path::{Component, Path, PathBuf},
        sync::Arc,
        time::Duration,
    },
    threadpool::ThreadPool,
    utils::*,
    verify_site::*,
};

mod utils;
mod verify_site;

fn main() -> std::io::Result<()> {
    let site_dir = Path::new("site");
    let index_path = site_dir.join("index.html");

    handle_site(site_dir, &index_path)?;

    const ADDRESS: &str = "127.0.0.1";
    const PORT: &str = "4242";
    let listener = TcpListener::bind(format!("{ADDRESS}:{PORT}"))?;
    println!("Server running on http://{ADDRESS}:{PORT}");

    let pool = ThreadPool::new(8);
    let listener = Arc::new(listener);

    for stream in listener.incoming().flatten() {
        pool.execute(move || {
            handle_client(stream);
        });
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

    let path = match sanitize_path(path.unwrap()) {
        Some(p) => p,
        None => {
            let _ = respond_400(&mut stream);
            return;
        }
    };

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

fn sanitize_path(request_path: &str) -> Option<PathBuf> {
    let base = Path::new("site");
    let mut safe_path = PathBuf::new();

    let clean_path = request_path.trim_start_matches('/');

    for component in Path::new(clean_path).components() {
        match component {
            Component::Normal(comp) => safe_path.push(comp),
            Component::CurDir | Component::ParentDir => continue,
            _ => continue,
        }
    }

    if safe_path.as_os_str().is_empty() {
        safe_path.push("index.html");
    }

    let full_path = base.join(safe_path);

    match full_path.canonicalize() {
        Ok(resolved) => {
            let root = base.canonicalize().ok()?;
            if resolved.starts_with(&root) {
                Some(resolved)
            } else {
                None
            }
        }
        Err(_) => None,
    }
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
