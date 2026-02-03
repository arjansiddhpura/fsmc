use crate::graph::FsmGraph;

pub fn get_enum_name(name: &str) -> String {
    let mut state_name = String::new();
    state_name.push_str(&format!("STATE_{}", name));
    state_name
}

fn generate_terminal_helper(graph: &FsmGraph) -> String {
    let mut code = String::from("int is_terminal(State s) {\n    switch (s) {\n");

    for state in &graph.states {
        // If state has no transitions, it is terminal
        if state.transitions.is_empty() {
            code.push_str(&format!(
                "        case {}: return 1;\n",
                get_enum_name(&state.name)
            ));
        }
    }

    // Default case (all other states) returns 0
    code.push_str("        default: return 0;\n");
    code.push_str("    }\n    return 0;\n}\n\n");
    code
}

fn generate_event_helper(graph: &FsmGraph) -> String {
    let mut code = String::new();
    code.push_str("void print_available_events(State s) {\n");
    code.push_str("     printf(\"   [Options: \");\n");
    code.push_str("     switch (s) {\n");

    for state in &graph.states {
        code.push_str(&format!("        case {}:\n", get_enum_name(&state.name)));
        code.push_str("            printf(\"");

        // Iterate over all transitions for this state
        for (i, trans) in state.transitions.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(&trans.event);
        }

        code.push_str("\");\n");
        code.push_str("            break;\n");
    }

    code.push_str("    }\n");
    code.push_str("    printf(\"]\\n\");\n}\n\n");
    code
}

pub fn generate_c(graph: &FsmGraph) -> String {
    let mut code = String::new();

    // Header
    code.push_str("#include <stdio.h>\n#include <string.h>\n\n");

    // 1. Generate Enum
    code.push_str("typedef enum{\n");
    for state in &graph.states {
        code.push_str(&format!("    {},\n", get_enum_name(&state.name)));
    }
    code.push_str("} State;\n\n");

    // 2. Generate Helpers
    code.push_str(&generate_event_helper(graph));
    code.push_str(&generate_terminal_helper(graph));

    // 3. Generate Transition Function
    code.push_str("State next_state(State current, const char* event) {\n");
    code.push_str("    switch (current) {\n");

    for (_id, state) in graph.states.iter().enumerate() {
        code.push_str(&format!("        case {}:\n", get_enum_name(&state.name)));
        for trans in state.transitions.iter() {
            code.push_str(&format!(
                "            if (strcmp(event, \"{}\") == 0) return {};\n",
                &trans.event,
                get_enum_name(&graph.states[trans.target].name)
            ));
        }
        code.push_str("            break;\n");
    }

    code.push_str("    }\n    return (State)-1;\n}\n\n");

    // 4. Helper function to print state names
    code.push_str("const char* state_name(State s) {\n");
    code.push_str("     switch (s) {\n");

    for (_id, state) in graph.states.iter().enumerate() {
        code.push_str(&format!(
            "        case {}: return \"{}\";\n",
            get_enum_name(&state.name),
            &state.name
        ));
    }

    code.push_str("    }\n    return \"Unknown\";\n}");

    // 5. Append the boilerplate Helper and Main
    code.push_str(BOILERPLATE);

    code
}

const BOILERPLATE: &str = r#"

int main() {
    State current = 0; // Assuming 0 is initial
    char buffer[100];
    
    printf("FSM Started...\n");

    while(1) {
        printf("   Current State: %s\n", state_name(current));

        if (is_terminal(current)) {
            printf(">> Final state reached. Terminating.\n");
            break;
        }
        
        print_available_events(current);

        printf(">> ");
        if (scanf("%99s", buffer) != 1) break;
        
        State next = next_state(current, buffer);
        if (next != (State)-1) {
            printf(">> Transitioned: %s -> %s\n", state_name(current), state_name(next));
            current = next;
        } else {
            printf(">> Invalid event. Stayed in %s.\n", state_name(current));
        }
    }
    return 0;
}
"#;
