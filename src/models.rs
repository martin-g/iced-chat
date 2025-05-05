use iced::{widget, Element, Length, Subscription, Task};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, row, scrollable, text, text_input, Column, Row};
use crate::{server, Message};
use crate::models::ChatMessage::{Me, Them};

#[derive(Debug)]
enum ChatMessage {
    Me(String),
    Them(String),
}

#[derive(Default, Debug)]
pub(crate) struct ChatApp {
    messages: Vec<ChatMessage>,
    input_value: String,
}

impl ChatApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                messages: Vec::new(),
                input_value: String::new(),
            },
            widget::focus_next(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        dbg!("update", &message);
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                Task::none()
            }
            Message::SendPressed => {
                if !self.input_value.trim().is_empty() {
                    self.messages.push(Me(self.input_value.clone()));
                    self.input_value.clear();
                }
                widget::focus_previous()
            },
            Message::BuddySent(msg) => {
                dbg!("Buddy sent", &msg);
                self.messages.push(Them(msg.clone()));
                Task::none()
            },
            Message::Server => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(Self::messages_container(self))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Bottom)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(server::run)
    }

    fn messages_container(&self) -> Element<'_, Message> {
        let messages: Element<_> = self
            .messages
            .iter()
            .fold(Column::new().width(Length::Fill).spacing(10), |column, message| {
                let message_row = Row::new()
                    .padding(10)
                    .width(Length::Fill)
                    .spacing(10);
                let row_column = Column::new()
                    .width(Length::Fill)
                    .padding(10)
                    .spacing(10);
                let txt = match message {
                    Me(txt) => row_column.align_x(Horizontal::Left).push(text(txt)),
                    Them(txt) => row_column.align_x(Horizontal::Right).push(text(txt)),
                };
                column.push(message_row.push(txt))
            })
            .into();

        let scrollable = scrollable(messages)
            .spacing(10);

        Column::new()
            .spacing(20)
            .push(scrollable)
            .push(row([self.msg_input(), self.send_button()]).spacing(10))
            .into()

    }

    fn msg_input(&self) -> Element<'_, Message> {
        text_input(
            "Type a message...",
            &self.input_value,
        )
            .on_input(Message::InputChanged)
            .padding(10)
            .size(16)
            .into()
    }

    fn send_button(&self) -> Element<'_, Message> {
        button("Send")
            .padding(10)
            .on_press(Message::SendPressed)
            .into()
    }
}