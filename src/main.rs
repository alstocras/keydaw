// imports!!
use iced::*;
use winit::*;

// gives it a default value
#[derive(Default)]
// the notes and time vars
struct AppState {
    
}

// view renderer
fn view(state: &AppState) -> Element<Messages>{widget::text("hello there").into()}

// lets it duplicate
#[derive(Clone)]
// the different kinds of interactions the user can have
enum Message {
    IncTime,
    DecTime,
    IncPitch,
    DecPitch,
    Quit,
}
