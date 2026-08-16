// imports
use iced::*;
use std::*;

// timeline struct
#[derive(Debug, Default)]
struct Timeline<T> {
    line: collections::HashMap<u64, f64>,
}

// timeline functions
impl<T> Timeline<T> {
    // updates everything
    fn update(timeline: &mut Timeline<T>, mess: Message) {
        match mess {
            Message::Quit => exit::<Task<T>>(),
        };
    }
    fn view(timeline: &Timeline<T>) -> Element<'_, Message> {
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
    run(Timeline::<Task<T>>::update, Timeline::view)
}
