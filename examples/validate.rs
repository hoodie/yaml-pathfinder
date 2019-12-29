use yaml_pathfinder::{open_yaml, validator::Validator, PathFinder as _, Yaml};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calendar = open_yaml("./examples/events.yml")?;

    let events = calendar.get_vec("events").unwrap();
    let validator = Validator::new()
        .check(|yml: &Yaml| yml.get_string("name"))
        .check(|yml: &Yaml| yml.get_int("num"))
        .require("/date")
        .fin();

    for event in events.iter() {
        dbg!(validator.validate(&event));
    }

    Ok(())
}
