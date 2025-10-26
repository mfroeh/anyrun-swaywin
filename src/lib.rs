use std::error::Error;

use abi_stable::std_types::{RString, RVec};
use anyrun_plugin::*;
use freedesktop_desktop_entry::{DesktopEntry, unicase::Ascii};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use swayipc::{Connection, Node, NodeType};

struct State {
    windows: Vec<(Node, Option<DesktopEntry>)>,
    locales: Vec<String>,
}

#[init]
fn init(_config_dir: RString) -> State {
    init_state().expect("init state")
}

fn init_state() -> Result<State, Box<dyn Error>> {
    let root = Connection::new()?.get_tree()?;
    let mut windows = Vec::new();
    let mut stack = vec![root];
    while let Some(top) = stack.pop() {
        if top.node_type == NodeType::Con {
            windows.push(top.clone());
        }
        stack.extend_from_slice(&top.nodes);
    }

    let locales = freedesktop_desktop_entry::get_languages_from_env();
    let desktop_entries = freedesktop_desktop_entry::desktop_entries(&locales);
    let windows: Vec<_> = windows
        .into_iter()
        .map(|w| {
            if let Some(app_id) = w.app_id.clone() {
                (
                    w,
                    freedesktop_desktop_entry::find_app_by_id(
                        &desktop_entries,
                        Ascii::new(&app_id),
                    )
                    .cloned(),
                )
            } else {
                (w, None)
            }
        })
        .collect();

    Ok(State { locales, windows })
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Windows".into(),
        icon: "window".into(),
    }
}

// https://github.com/anyrun-org/anyrun/blob/master/plugins/applications/src/lib.rs
#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    let words: Vec<_> = input.split_whitespace().collect();
    let matcher = SkimMatcherV2::default();
    let mut scored_windows: Vec<_> = state
        .windows
        .iter()
        .filter_map(|(win, entry)| {
            let scores = words.iter().map(|w| {
                let mut application_name = win.app_id.clone().unwrap_or("".to_string());
                if let Some(entry) = entry {
                    if let Some(name) = entry.name(&state.locales) {
                        application_name = name.to_string();
                    }
                }

                let full_window_name = win.name.clone().unwrap_or("".into());
                let mut score = matcher
                    .fuzzy_match(&application_name, w)
                    .unwrap_or_default()
                    * 2;
                score += matcher
                    .fuzzy_match(&full_window_name, w)
                    .unwrap_or_default();
                if score == 0 {
                    return None;
                }
                Some(score)
            });
            let mut score = 0;
            for s in scores {
                if let Some(s) = s {
                    score += s;
                } else {
                    return None;
                }
            }
            Some((win, entry, score))
        })
        .collect();
    scored_windows.sort_by_key(|f| -f.2);

    scored_windows
        .into_iter()
        .map(|(win, entry, _)| {
            let mut application_name = win.app_id.clone().unwrap_or("".to_string());
            if let Some(entry) = entry {
                if let Some(name) = entry.name(&state.locales) {
                    application_name = name.to_string();
                }
            }

            let full_window_title = win.name.clone().unwrap_or("".into());
            Match {
                id: Some(win.id as u64).into(),
                icon: entry
                    .as_ref()
                    .and_then(|e| e.icon())
                    .map(RString::from)
                    .into(),
                title: application_name.into(),
                description: Some(full_window_title.into()).into(),
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
