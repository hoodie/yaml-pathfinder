use yaml_pathfinder::PathFinder;
use yaml_rust::YamlLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let folders = YamlLoader::load_from_str(
        r#"
home:
    hendrik:
        code:
            rust:
                - asciii
                - notify-rust
                - yaml_pathfinder
"#,
    )?
    .remove(0);

    let path_style = folders.get_str("/home/hendrik/code/rust/1")?;
    println!("{:?}", path_style);

    let json_style = folders.get_str("home.hendrik.code.rust.2")?;
    println!("{:?}", json_style);

    let full_hash = folders.get_hash("home.hendrik.code")?;
    println!("{:?}", full_hash);

    Ok(())
}
