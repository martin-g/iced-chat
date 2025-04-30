use iced::widget::{
    Button, Column, Container,
    Row, Scrollable, Text, TextInput,
};
use iced::{application, Element, Length, Settings, Theme};
use iced::alignment::{Horizontal, Vertical};

pub fn main() -> iced::Result {
    application("Chat Application", update, view)
        .settings(Settings::default())
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Default, Debug)]
struct ChatApp {
    messages: Vec<String>,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SendPressed,
}

fn update(chat: &mut ChatApp, message: Message) {
    match message {
        Message::InputChanged(value) => {
            chat.input_value = value;
        }
        Message::SendPressed => {
            if !chat.input_value.trim().is_empty() {
                chat.messages.push(chat.input_value.clone());
                chat.input_value.clear();
            }
        }
    }
}

fn view(chat: &ChatApp) -> Element<Message> {
    let input = TextInput::new(
        "Type a message...",
        &chat.input_value,
    )
        .on_input(Message::InputChanged)
        .padding(10)
        .size(16);

    let send_button = Button::new(Text::new("Send"))
        .padding(10)
        .on_press(Message::SendPressed);

    let messages: Element<_> = chat
        .messages
        .iter()
        .fold(Column::new().spacing(10), |column, message| {
            column.push(Text::new(message)).width(Length::Fill)
        })
        .into();

    let scrollable = Scrollable::new(messages)
        .spacing(10);

    let content = Column::new()
        .spacing(20)
        .push(scrollable)
        .push(Row::new().spacing(10).push(input).push(send_button).align_y(Vertical::Bottom));

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}
