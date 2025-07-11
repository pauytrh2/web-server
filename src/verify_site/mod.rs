use std::{
    fs::{File, create_dir_all},
    io::{Result, Write},
    path::Path,
};

const DEFAULT_SITE: &[u8] = b"<!DOCTYPE html>
<html>
<head><title>Welcome</title></head>
<body>
<h1>Hello from HTTP Server</h1>
<p>This is the default index.html page. You may replace it with any static site.</p>
</body>
</html>";

pub fn handle_site(site_dir: &Path, index_path: &Path) -> Result<()> {
    if !site_dir.exists() {
        create_dir_all(site_dir)?;
        println!("Created 'site' directory");
    } else {
        println!("Found 'site' directory");
    }

    if !index_path.exists() {
        let mut file = File::create(index_path)?;
        file.write_all(DEFAULT_SITE)?;
        println!("Created default 'site/index.html'");
    } else {
        println!("Found 'site/index.html'")
    }

    Ok(())
}
