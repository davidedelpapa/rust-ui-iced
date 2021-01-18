use iced::{Element, Sandbox, Settings, Text};

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        MyApp
    }

    fn title(&self) -> String {
        String::from("Hello, Iced")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}