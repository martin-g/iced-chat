mod models;

use iced::widget::{button, text_input, Column, Container, Row, Scrollable, Text};
use iced::{application, Element, Length, Settings, Theme};
use iced::alignment::{Horizontal, Vertical};
use crate::models::ChatApp;

#[tokio::main]
pub async fn main() -> iced::Result {
    application("Chat Application", update, view)
        .settings(Settings::default())
        .theme(|_| Theme::Dark)
        .centered()
        .run()
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
    
    Container::new(messages_container(chat))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Bottom)
        .into()
}

fn messages_container(chat: &ChatApp) -> Element<'_, Message> {
    let messages: Element<_> = chat
        .messages
        .iter()
        .fold(Column::new().spacing(10), |column, message| {
            column.push(Text::new(message)).width(Length::Fill)
        })
        .into();

    let scrollable = Scrollable::new(messages)
        .spacing(10);

    Column::new()
        .spacing(20)
        .push(scrollable)
        .push(Row::new().spacing(10).push(msg_input(chat)).push(send_button(chat)))
        .into()

}

fn msg_input(chat: &ChatApp) -> Element<'_, Message> {
    text_input(
        "Type a message...",
        &chat.input_value,
    )
        .on_input(Message::InputChanged)
        .padding(10)
        .size(16)
        .into()
}

fn send_button(_chat: &ChatApp) -> Element<'_, Message> {
    button("Send")
        .padding(10)
        .on_press(Message::SendPressed)
        .into()

}