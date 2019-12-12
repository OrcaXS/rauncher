use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color, Column,
    Container, Element, HorizontalAlignment, Image, Length, Radio, Row,
    Sandbox, Scrollable, Settings, Slider, Text, TextInput, Font
};

mod dir_index;
mod errors;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}
pub fn main() {
    env_logger::init();

    Counter::run(Settings::default())
}

struct Counter {
    // The counter value
    value: i32,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Counter {
        Counter {
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        format!("test")
    }

    fn view(&mut self) -> Element<Message> {
        let cjk_font = Font::Default;
        // We use a column: a simple vertical layout
        Column::new()
            .push(
                Text::new(dir_index::get_app_dir().unwrap().dir.to_str().unwrap_or("none"))
            )
            .push(
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                Text::new("测试".to_string()).size(50).font(cjk_font),
            )
            .push(
                // We show the value of the counter here
                Text::new(self.value.to_string()).size(50),
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            ).into()
    }
    
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}
