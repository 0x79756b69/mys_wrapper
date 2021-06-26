
## mys-wrapper
safety rust-mysql-simple wrapper.

## How to use
1. Import
```toml
mys_wrapper = { git="https://github.com/0x79756b69/mys_wrapper", branch="main" }
```
2. Use
```rust
fn main() {
    use crate::{exec};
    let maps:Vec<HashMap<String, String>> = exec(
        "mysql://root:{PSWD}@localhost:3306/{DB_NAME}",
        "SELECT * FROM users WHERE name=? AND pswd = ?",
        vec!["Bob".to_string(), "PASSWORD_123".to_string()]).unwrap();
    // This output hashmaps.
    println!("{:?}", maps);
}
```

## outlook
- Improve efficiency
