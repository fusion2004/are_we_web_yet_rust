use gotham::state::State;

const GREETING: &'static str = "Welcome to my rust service!";

pub fn handler(state: State) -> (State, &'static str) {
    (state, GREETING)
}
