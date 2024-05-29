pub mod api;
pub mod chat;
mod storage;
use env_logger;
use std::io;

fn main() {
    env_logger::init();
}

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_string())
}
