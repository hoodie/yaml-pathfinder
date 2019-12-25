use yaml_pathfinder::PathFinder as _;
use yaml_pathfinder::{error::ValidationResult, open_yaml, Yaml};

fn validate_event(event: &Yaml) {
    let mut validation = ValidationResult::new();
    validation.validate_field("name", event.get_str("name"));
    validation.validate_field("num", event.get_int("num"));
    validation.require_field("date", event.get_str("date"));
    dbg!( validation);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calendar = open_yaml("./examples/events.yml")?;

    let events = calendar.get_vec("events").unwrap();
    for event in events.iter() {
        validate_event(event);
    }

    Ok(())
}
