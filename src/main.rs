// imports
use iced::*;
use std::*;

// timeline struct
#[derive(Debug, Default)]
struct Timeline {
    line: collections::HashMap<u64, f64>,
}

// timeline functions
impl Timeline {
    // updates everything
    fn update(timeline: &mut Timeline, mess: Message) -> Task<Message> {
        match mess {
            Message::Quit => process::exit(0),
        };
    }
    fn view(timeline: &Timeline) -> Element<'_, Message> {
        widget::button("quit").on_press(Message::Quit).into()
    }
}

// kinds of interactions
#[derive(Debug, Clone)]
enum Message {
    Quit,
}

// main runner
fn main() -> Result {
    run(Timeline::update, Timeline::view)
}
