A simple crate for generating GraphViz dot directed trees, 
    based on an arbitrary tree structure.
A tree can be any struct that implements:
    - `std::string::ToString`
    - `std::hash::Hash`
    - and `TreeVizNode`
Currently, this crate does not support recursive elements within a tree.

This crate aims to be dependency free, and fast.

