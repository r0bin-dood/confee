# Welcome to confee!

Parse your simple configuration files in an ergonomic and safe way! :)

Available in [crates.io](https://crates.io/crates/confee)

## Usage

To use confee, your configurations should follow this format:

[key][delim][value]\n

### Where:

- **`key`**: Any sequence of printable characters.
- **`delim`**: A delimiter character, such as `':'`, `' '`, `'='`, etc.
- **`value`**: Any sequence of printable characters.

Each new line represents a new key-value pair.

### Example Configuration

https://github.com/r0bin-dood/confee/blob/main/examples/example.conf

```
log: stdout
dir: ./example/
addr: 127.0.0.1
port: 8080
```

### Example Usage

https://github.com/r0bin-dood/confee/blob/main/examples/main.rs

```rust
use std::env;
use confee::conf::*;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

macro_rules! conf_defaults {
    () => {
        [
            ("log".to_string(), "stdout".to_string()),
            ("dir".to_string(), "/var/www/html/".to_string()),
            ("addr".to_string(), "127.0.0.1".to_string()),
            ("port".to_string(), "8080".to_string()),
        ]
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut conf = Conf::from(conf_defaults!());
    match conf.with_file(&args[1]).update() {
        Ok(_) => println!("Successfully updated configuration!"),
        Err(e) => panic!("Error updating configuration: {}", e),
    }

    
    let dir: PathBuf = conf.get("dir").unwrap();
    let addr: IpAddr = conf.get("addr").unwrap();
    let port: u16 = conf.get("port").unwrap();
    
    println!("log:  {}", conf["log"]);
    println!("dir:  {}", dir.to_string_lossy());
    println!("addr: {}", addr);
    println!("port: {}", port);

    dbg!(conf);
}
```

## Documentation

Run `cargo doc --open` to view in-code docummentation locally, or visit:

https://docs.rs/confee/latest/confee/index.html
