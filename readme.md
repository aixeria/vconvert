# vconvert
convert vulnus to soundspace maps. (or vice versa).

[docs.rs](https://docs.rs/vconvert/0.1.0/vconvert/)
[crates.io](https://crates.io/crates/vconvert)

# usage
Firstly add in your cargo.toml:
```toml
vconvert = "0.1.0"
```
Then in your code:
```rust
use vconvert::vulnus;

fn main() {


    let mut vulnus_map = serde_json::from_slice::<vulnus::Map>(
        include_bytes!("../vmap.json") // or however you get your map
    ).expect("unable to parse map");
    // then do what ever you want with it
}
```

# vconvert-ui
VConvert-ui is a ui version of this tool, which allows novices of the command line to instead use a gui built with tauri.
## images
![](https://i.imgur.com/ShXjssR.png)
![](https://i.imgur.com/igejuh7.png)
# examples
There are some examples in the [examples](./examples/) folder.
