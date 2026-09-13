use std::{fs, path::Path};

use petgraph::{Graph, graph::NodeIndex};
use ptree::print_tree;
use std::io::Result as IoResult;

use crate::config::ShowArgs;

type TestsGraph = Graph<String, ()>;

fn build_dir(graph: &mut TestsGraph, current_path: &Path, parent: NodeIndex) -> IoResult<()> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if entry.file_type()?.is_dir() {
            let node = graph.add_node(name);
            build_dir(graph, &path, node)?;
            if graph.neighbors(node).next().is_some() {
                graph.add_edge(parent, node, ());
            }
        } else if name.ends_with(".spec.lua") {
            let node = graph.add_node(name);
            graph.add_edge(parent, node, ());
        }
    }

    Ok(())
}

pub fn handle_show(_args: &ShowArgs) -> IoResult<()> {
    let mut graph = TestsGraph::new();
    let root = graph.add_node(".".to_string());

    build_dir(&mut graph, Path::new("."), root)?;
    print_tree(&(&graph, root))?;

    Ok(())
}
