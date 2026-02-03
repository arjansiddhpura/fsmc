#[derive(Debug)]
pub struct Machine {
    pub name: String,
    pub states: Vec<State>,
}

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub transitions: Vec<Transition>,
}

#[derive(Debug)]
pub struct Transition {
    pub event: String,
    pub target: String,
}
