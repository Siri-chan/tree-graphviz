//! A simple crate for generating GraphViz dot directed trees, based on an arbitrary tree structure

use std::{hash::{DefaultHasher, Hash, Hasher}, string::ToString};

pub trait TreeVizNode where Self: ToString {
    fn children(&self) -> Vec<Self> where Self: Sized;
    
}

pub fn draw_nodes<T: TreeVizNode + Hash>(graph_name: &String, node: T) -> Result<String, ()> {
    let out = format!("graph {} {{", graph_name);
    Ok(format!("{}\n{}", out, draw_node(None, node)))
}

fn draw_node<T: TreeVizNode + Hash>(parent_hash: Option<u64>, node: T) -> String {
    let mut hasher = DefaultHasher::new();
    node.hash(&mut hasher);
    let hash = hasher.finish();
    let mut out = format!("{} [label=\"{}\"];\n", hash, sanitize(node.to_string()));
    if let Some(parent) = parent_hash {
        out = format!("{} {} -> {};\n", out, parent, hash);
    }
    for child in node.children() {
        out = format!("{}{};\n", out, draw_node(Some(hash), child));
    }
    out
}

fn sanitize(s: String) -> String {
    s.replace(|c: char| !c.is_ascii(), "").replace('"', "\\\"")
}
