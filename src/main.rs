use iced::*;
struct Timeline {
    time: u64,
    note: char,
}
enum Message {
    AddNote,
    RemoveNote,
}
