mod models;
mod server;

use crate::models::ChatApp;
use iced::{application, Settings, Theme};

#[tokio::main]
pub async fn main() -> iced::Result  {
    
    application(/*ChatApp::new*/"Chat Application", ChatApp::update, ChatApp::view)
        .settings(Settings::default())
        .theme(|_| Theme::Dark)
        .subscription(ChatApp::subscription)
        .centered()
        .run_with(ChatApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    BuddySent(String),
    SendPressed,
    Server
}


