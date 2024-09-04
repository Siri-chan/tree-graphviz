//! A simple crate for generating GraphViz dot directed trees, based on an arbitrary tree structure

use std::{collections::HashSet, hash::{DefaultHasher, Hash, Hasher}, string::ToString};

pub trait TreeVizNode where Self: ToString {
    fn children(&self) -> Vec<Self> where Self: Sized;
    
}

pub fn draw_nodes<T: TreeVizNode + Hash>(graph_name: &String, node: T) -> Result<String, ()> {
    let mut out = format!("digraph {} {{", graph_name);
    out = format!("{}\n{}", out, draw_node(None, node, &mut HashSet::new()));
    out = out.lines().map(|ln| ln.trim()).filter(|ln| *ln != ";").collect();
    out.push('}');
    Ok(out)
}

fn draw_node<T: TreeVizNode + Hash>(parent_hash: Option<u64>, node: T, hashes: &mut HashSet<u64>) -> String {
    let mut hasher = DefaultHasher::new();
    node.hash(&mut hasher);
    let mut hash = hasher.finish();
    while hashes.contains(&hash) {
        hash += 1;
    }
    hashes.insert(hash);
    let mut out = format!("{} [label=\"{}\"];\n", hash, sanitize(node.to_string()));
    if let Some(parent) = parent_hash {
        out = format!("{}{} -> {};\n", out, parent, hash);
    }
    for child in node.children() {
        out = format!("{}{};\n", out, draw_node(Some(hash), child, hashes));
    }
    out
}

fn sanitize(s: String) -> String {
    s.replace(|c: char| !c.is_ascii(), "").replace('"', "\\\"")
}
