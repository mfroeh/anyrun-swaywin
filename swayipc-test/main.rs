use std::collections;

use swayipc::{Connection, Event, EventType, Fallible, NodeType};

fn main() -> Fallible<()> {
    println!(
        "{:?}",
        windows
            .iter()
            .map(|n| n.name.clone().unwrap())
            .collect::<Vec<_>>()
    );
    Ok(())
}
