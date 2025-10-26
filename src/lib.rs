use std::error::Error;

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use swayipc::{Connection, Node, NodeType};

#[init]
fn init(config_dir: RString) -> Vec<Node> {
    collect_windows().expect("collect sway windows")
}

fn collect_windows() -> Result<Vec<Node>, Box<dyn Error>> {
    let root = Connection::new()?.get_tree()?;
    let mut windows = Vec::new();
    let mut stack = vec![root];
    while let Some(top) = stack.pop() {
        if top.node_type == NodeType::Con {
            windows.push(top.clone());
        }
        stack.extend_from_slice(&top.nodes);
    }
    Ok(windows)
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Sway Windows".into(),
        icon: "window-new".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, windows: &[Node]) -> RVec<Match> {
    let matcher = SkimMatcherV2::default();
    let matching_windows = windows.iter().filter(|n| {
        matcher
            .fuzzy_match(&n.name.clone().unwrap_or("".into()), &input)
            .is_some()
    });
    matching_windows
        .map(|n| {
            let window_title = n.name.clone().unwrap_or("".into());
            Match {
                id: None.into(),
                icon: None.into(),
                title: window_title.clone().into(),
                description: Some(window_title.into()).into(),
                use_pango: false,
            }
        })
        .collect()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    // Handle the selected match and return how anyrun should proceed
    HandleResult::Close
}
