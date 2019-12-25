use yaml_pathfinder::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let folders = Yaml::parse(
        r#"
home:
    hendrik:
        code:
            rust:
                - asciii
                - notify-rust
                - yaml_pathfinder
"#,
    )?;

    let path_style = folders.get_str("/home/hendrik/code/rust/1")?;
    println!("{:?}", path_style);

    let json_style = folders.get_str("home.hendrik.code.rust.2")?;
    println!("{:?}", json_style);

    let full_hash = folders.get_hash("home.hendrik.code")?;
    println!("{:?}", full_hash);

    Ok(())
}
