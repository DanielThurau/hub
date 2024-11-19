#[derive(Debug, Clone)]
pub enum Message {
    Twilio(TwilioMessage),
    App(AppMessage),
}

#[derive(Debug, Clone)]
pub struct TwilioMessage {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct AppMessage {
    pub message: String,
}

impl Message {
    pub fn get_queue_type(&self) -> MessageType {
        match self {
            Message::Twilio(_) => MessageType::Twilio,
            Message::App(_) => MessageType::App,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageType {
    Twilio,
    App,
}