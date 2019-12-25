# Yaml PathFinder

A tiny convenience library that allows you to access values from structured data (e.g. Yaml) dynamically by path.

## Usage

You can either import the prelude, which contains our wrapper type `Yaml`;

```rust
use yaml_pathfinder::prelude::*;
let folders = Yaml::parse(r#"
home:
    hendrik:
        code:
            rust:
                - asciii
                - notify-rust
                - yaml_pathfinder
"#)?;

let path_style = folders.get_str("/home/hendrik/code/rust/1")?;
println!("{:?}", path_style);
...
```

You can also just import the `PathFinder` trait itself, it is implemented on `yaml_rust::Yaml` too.

```rust
use yaml_pathfinder::PathFinder;
use yaml_rust::YamlLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let folders = YamlLoader::load_from_str(r#"
home:
    hendrik:
        code:
            rust:
                - asciii
                - notify-rust
                - yaml_pathfinder
"#)?.remove(0);

    let path_style = folders.get_str("/home/hendrik/code/rust/1")?;
    println!("{:?}", path_style);
    ...
}
```
