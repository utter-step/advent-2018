use std::path::Path;

use advent_utils::parse_file;

use crate::graph::InstructionGraph;
use crate::instruction::OperationOrder;

pub(crate) fn parse_graph<P: AsRef<Path>>(p: P) -> InstructionGraph {
    parse_file(p).unwrap().into_iter().fold(
        InstructionGraph::new(),
        |mut graph, order: OperationOrder| {
            graph.add_link(order.first, order.second);

            graph
        },
    )
}
