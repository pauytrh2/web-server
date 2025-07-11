use {
    std::{net::TcpListener, path::Path, sync::Arc},
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
