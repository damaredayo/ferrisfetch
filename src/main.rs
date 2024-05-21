mod logo;
mod modules;
mod print;

pub struct State {
    sys: sysinfo::System,
}

fn main() {
    let mut state = State {
        sys: sysinfo::System::new(),
    };

    state.print_with_logo();
}
