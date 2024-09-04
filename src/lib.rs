#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unstable_features)]

//! A simple crate for generating GraphViz dot directed trees,
//!     based on an arbitrary tree structure.
//! A tree can be any struct that implements:
//!     [std::string::ToString], [std::hash::Hash] and [TreeVizNode].
//! Currently, this crate does not support recursive elements within a tree.

use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
    string::ToString,
};

/// A trait that represents a node in an arbitrary tree structure.
/// To use this with [draw_nodes], you also need to implement [std::hash::Hash].
pub trait TreeVizNode
where
    Self: ToString,
{
    /// Returns a vector containing the sub-nodes that are children of this node.
    fn children(&self) -> Vec<Self>
    where
        Self: Sized;
}

/// Returns a visualisation of the tree with the root node `node` in the GraphViz DOT format, as a
/// string.
/// - `graph_name` is the name of the graph, which will have spaces and non-ascii characters
///     removed to comply with DOT's format restrictions.
pub fn draw_nodes<T: TreeVizNode + Hash>(graph_name: &str, node: T) -> String {
    let graph_name: String = graph_name
        .to_owned()
        .chars()
        .filter(|c| *c != ' ' && c.is_ascii())
        .collect();
    let mut out = format!("digraph {} {{", graph_name);
    out = format!("{}\n{}", out, draw_node(None, node, &mut HashSet::new()));
    out = out
        .lines()
        .map(|ln| ln.trim())
        .filter(|ln| *ln != ";")
        .collect();
    out.push('}');
    out
}

fn draw_node<T: TreeVizNode + Hash>(
    parent_hash: Option<u64>,
    node: T,
    hashes: &mut HashSet<u64>,
) -> String {
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
    s.replace(|c: char| !c.is_ascii(), "")
        .replace('"', "\\\"")
        .chars()
        .filter(|c| c.is_ascii())
        .collect()
}
