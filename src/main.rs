// imports
use iced::*;
use std::*;

// timeline struct
#[derive(Debug, Default)]
struct Timeline {
    // timeline map <time, pitch>
    line: collections::HashMap<u64, f64>,
}

// timeline functions
impl Timeline {
    // updates everything
    fn update(timeline: &mut Timeline, mess: Message) -> Task<Message> {
        match mess {
            Message::Quit => process::exit(0),
            Message::IncTime => {
                timeline.line.insert(0, 0.0);
                println!("{:?}", timeline.line)
            }
        };
        return Task::none();
    }

    // shows the app
    fn view(timeline: &Timeline) -> Element<'_, Message> {
        widget::row![
            widget::button("quit").on_press(Message::Quit),
            widget::button("increment time").on_press(Message::IncTime)
        ]
        .into()
    }
}

// kinds of interactions
#[derive(Debug, Clone)]
enum Message {
    IncTime,
    Quit,
}

// main runner
fn main() -> Result {
    run(Timeline::update, Timeline::view)
}
