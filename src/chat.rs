use chrono::{DateTime, Local, Timelike};
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize, Clone)]
pub enum Sender {
    User,
    Model,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub sender: Sender,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

impl ChatMessage {
    pub fn new(sender: Sender, text: String) -> Self {
        ChatMessage {
            sender,
            content: text,
            timestamp: Local::now(),
        }
    }
}

impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = self.timestamp.time();
        write!(
            f,
            "[{:02}:{:02}:{:02}] {}: {}",
            time.hour(),
            time.minute(),
            time.second(),
            match self.sender {
                Sender::User => "User",
                Sender::Model => "Model",
            },
            self.content
        )
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: usize,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub timestamp: DateTime<Local>,
}

impl Chat {
    pub fn new(id: usize, model: String) -> Chat {
        Chat {
            id,
            model,
            messages: Vec::new(),
            timestamp: Local::now(),
        }
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
    }

    pub fn render(&self) {
        for message in &self.messages {
            println!("{}", message);
        }
    }
}
/*pub fn store(&self) -> HashMap<String(String, (String, String))> {
    self.messages
        .iter()
        .map(|message| {
            (
                message.timestamp.to_rfc3339(),
                (
                    message.content.clone(),
                    match message.sender {
                        Sender::User => "User".to_string(),
                        Sender::Model => self.model.to_string(),
                    },
                ),
            )
        })
        .collect()
}

pub fn import(chat: &HashMap<String, (String, String)>) -> Chat {
    let mut new_chat = Chat::new(chat.get("model").unwrap().0.to_string());
    for (timestamp, (content, sender)) in chat {
        new_chat.add_message(Message {
            sender: match sender.as_str() {
                "User" => Sender::User,
                "Model" => Sender::Model,
                _ => panic!("Invalid sender"),
            },
            content: content.to_string(),
            timestamp: DateTime::parse_from_rfc3339(timestamp)
                .unwrap()
                .with_timezone(&Local),
        })
    }
    new_chat
} */
