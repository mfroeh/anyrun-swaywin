use std::error::Error;

use abi_stable::std_types::{RString, RVec};
use anyrun_plugin::*;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use swayipc::{Connection, Node, NodeType};

#[init]
fn init(_config_dir: RString) -> Vec<Node> {
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
    let mut scored_windows: Vec<_> = windows
        .iter()
        .filter_map(|n| {
            let score = matcher.fuzzy_match(&n.name.clone().unwrap_or("".into()), &input);
            if let Some(score) = score {
                return Some((n, score));
            }
            None
        })
        .collect();
    scored_windows.sort_by_key(|f| -f.1);

    scored_windows
        .into_iter()
        .map(|(n, _)| {
            let window_title = n.name.clone().unwrap_or("".into());
            let class = n.app_id.clone().unwrap_or("".into());
            Match {
                id: Some(n.id as u64).into(),
                icon: None.into(),
                title: class.into(),
                description: Some(window_title.into()).into(),
                use_pango: false,
            }
        })
        .collect()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    let mut connection = Connection::new().expect("get connection");
    connection
        .run_command(format!(
            "[con_id={}] focus",
            selection.id.expect("got no id")
        ))
        .expect("focus window");
    HandleResult::Close
}
