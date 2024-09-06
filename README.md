A simple crate for generating GraphViz dot directed trees, 
    based on an arbitrary tree structure.
A tree can be any struct that implements:
    - `std::string::ToString`
    - `std::hash::Hash`
    - and `TreeVizNode`
Currently, this crate does not support recursive elements within a tree.

This crate aims to be dependency free, and fast.
An optional `"async"` feature is available and provides an async variant of
    `draw_nodes` - `draw_nodes_async`, which will recurse through a 
    node's children concurrently.
This introduces a dependency on `futures`, but may be quicker, especially if
    futures is already in your dependency tree.

If you want to run tests on this crate, use `cargo test --all-features`, 
    or the async-related tests will not run.

