# 📚 K-Librarian

A simple invite system web server for Komga.<br />
Made on top of [Axum](https://github.com/tokio-rs/axum)

A basic and minimal clone of [Wizarr](https://github.com/wizarrrr/wizarr)

## Requirements
1. Node.js 18.x or higher
2. Rust 1.66 or higher
3. Redis
4. Komga server

## Usages
1. Install both Node/NPM and Rust
2. Clone this repository
3. [Configure](#configuration) your instances
4. Install dependencies for frontend: `npm install`
5. Build the packages by using `cargo build --release` to build optimized release version
6. Run the target file in `target/release/librarian`
7. Open: http://127.0.0.1:5148

## Configuration
The invite system use simple dotenv (`.env`) file to configure the server, you can see the `.env.example`
file, copy into `.env` and modify it.

```conf
# The host and port of the server
HOST=127.0.0.1
PORT=5148

# Set the token for authorization in the web ui, make sure it's secure!
# TOKEN=

### Komga configuration
# The host of the komga server
KOMGA_HOST=https://demo.komga.org
# The username of the komga server
KOMGA_USERNAME=demo@komga.org
# The password of the komga server
KOMGA_PASSWORD=demo

### Redis configuration
# Host and port of the redis server
REDIS_HOST=127.0.0.1
REDIS_PORT=6379
# Password of the redis server, uncomment if needed
# REDIS_PASS=
```

## Attribution

The icon/favicon/logo used by K-Librarian is a non-modified version of icon called **books icon** by Freepik: [Flaticon](https://www.flaticon.com/free-icons/books)
All rights reserved to the original creator.