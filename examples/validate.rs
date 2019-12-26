use yaml_pathfinder::PathFinder as _;
use yaml_pathfinder::{open_yaml, validator::Validator, Yaml};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calendar = open_yaml("./examples/events.yml")?;

    let events = calendar.get_vec("events").unwrap();
    let validator = Validator::new()
        .add_rule(|yml: &Yaml| yml.get_string("name"))
        .add_rule(|yml: &Yaml| yml.get_int("num"))
        .require("date")
        .fin();

    for event in events.iter() {
        dbg!(validator.validate(&event));
    }

    Ok(())
}
