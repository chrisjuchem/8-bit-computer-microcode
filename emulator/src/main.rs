use crate::state::State;
use microcode::programs::ADDITION;

mod state;

fn main() {
    let mut prog_state = State::new(ADDITION);
    let mut input = String::new();
    loop {
        println!("{}", prog_state);
        // std::io::stdin().read_line(&mut input).unwrap();
        if prog_state.step().is_err() {
            break;
        };
    }
}
