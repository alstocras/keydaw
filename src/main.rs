// imports
use iced::*;
use std::*;

// timeline struct
#[derive(Default)]
struct Timeline {
    line: collections::HashMap,
}

// timeline functions
impl Timeline {
    // updates everything
    fn update(timeline: &mut Timeline, mess: Message) {}
}

// main runner
fn main() -> Result {
    run(update, view);
}

// kinds of interactions
enum Message {
    Quit,
}
