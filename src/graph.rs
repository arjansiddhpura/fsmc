use crate::ast;
use std::collections::HashMap;

pub type StateId = usize;

#[derive(Debug)]
pub struct FsmGraph {
    pub states: Vec<StateNode>,
    pub initial_state: StateId,
}

#[derive(Debug)]
pub struct StateNode {
    pub name: String,
    pub transitions: Vec<TransitionEdge>,
}

#[derive(Debug)]
pub struct TransitionEdge {
    pub event: String,
    pub target: StateId,
}

impl FsmGraph {
    pub fn compile(machine: ast::Machine) -> Result<FsmGraph, String> {
        let mut states = Vec::new();
        let mut state_map: HashMap<String, StateId> = HashMap::new();

        // Pass 1: Collect all states and assign IDs
        for (i, machine_state) in machine.states.iter().enumerate() {
            state_map.insert(machine_state.name.clone(), i);
            states.push(StateNode {
                name: machine_state.name.clone(),
                transitions: Vec::new(),
            })
        }

        // Pass 2: Resolve transitions
        for (source_index, machine_state) in machine.states.iter().enumerate() {
            for trans in &machine_state.transitions {
                match state_map.get(&trans.target) {
                    Some(target_index) => states[source_index].transitions.push(TransitionEdge {
                        event: trans.event.clone(),
                        target: *target_index,
                    }),
                    None => return Err(format!("Undefined state: {}", trans.target)),
                }
            }
        }

        if states.is_empty() {
            return Err("Machine must have at least one state!".to_string());
        }

        // For now, assume first state (0) is always initial
        Ok(FsmGraph {
            states,
            initial_state: 0,
        })
    }
}
