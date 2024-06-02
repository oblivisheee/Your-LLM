use std::io::{self, Write};

pub fn read_line(prefix: Option<&str>) -> io::Result<String> {
    if let Some(prefix) = prefix {
        print!("{}", prefix);
    }
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_string())
}
