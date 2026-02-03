use crate::graph::FsmGraph;

pub fn generate_dot(graph: &FsmGraph) -> String {
    let mut output = String::from("digraph {\n");

    for (source_id, state) in graph.states.iter().enumerate() {
        for trans in state.transitions.iter() {
            let source = &graph.states[source_id].name;
            let target = &graph.states[trans.target].name;
            let event = &trans.event;
            let line = format!("    {source} -> {target} [label=\"{event}\"];\n");
            output.push_str(&line);
        }
    }

    output.push_str("}\n");
    output
}
