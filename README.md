> **A blazingly fast, minimal web server written in rust**

# Installation

verify rust and cargo are installed
```bash
rustc --version
cargo --version
```
clone, cd, format, and build the project
```bash
git clone https://github.com/pauytrh2/web-server
cd web-server
cargo fmt
cargo run
```
or in one line
```bash
git clone https://github.com/pauytrh2/web-server && cd web-server && cargo fmt && cargo run
```

# Usage
1. Replace the contents of the site folder with any static site you want to host (must have index.html as the root of the site)
2. run (```cargo fmt && cargo run```)
3. visit the address at the port in your web browser (if you set the ADDRESS variable to "172.0.0.1" and the PORT variable to "4242", you would visit: "172.0.0.1:4242")