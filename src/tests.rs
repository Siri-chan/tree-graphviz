use super::*;

#[derive(Debug, Clone, Hash)]
struct Tree1 {
    name: String,
    children: Vec<Tree1>,
}
impl ToString for Tree1 {
    fn to_string(&self) -> String {
        return self.name.clone();
    }
}
impl TreeVizNode for Tree1 {
    fn children(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        self.children.clone()
    }
}

#[test]
fn sync_correctness() {
    let tree1: Tree1 = Tree1 {
        name: String::from("Root"),
        children: vec![
            Tree1 {
                name: String::from("Child1"),
                children: vec![],
            },
            Tree1 {
                name: String::from("Child2"),
                children: vec![],
            },
        ],
    };
    let expected = r#"digraph Correct {11854684405404351146 [label="Root"];1720413889726253583 [label="Child1"];11854684405404351146 -> 1720413889726253583;11883667564293945662 [label="Child2"];11854684405404351146 -> 11883667564293945662;}"#;
    let result = draw_nodes("Correct", tree1);
    assert_eq!(expected, result);
}

#[test]
#[cfg(feature = "async")]
fn async_sync_parity() {
    use futures::executor::block_on;
    let tree1: Tree1 = Tree1 {
        name: String::from("Root"),
        children: vec![
            Tree1 {
                name: String::from("Child1"),
                children: vec![],
            },
            Tree1 {
                name: String::from("Child2"),
                children: vec![],
            },
        ],
    };
    let async_ = draw_nodes_async("Parity", tree1.clone());
    let sync = draw_nodes("Parity", tree1);
    let async_ = block_on(async_);
    assert_eq!(async_, sync);
}
