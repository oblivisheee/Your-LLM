use crate::{api::APIClient, chat::Chat};
use bincode;
use log::trace;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub api_clients: Vec<APIClient>,
    pub chats: Vec<Chat>,
}

impl Storage {
    pub fn new() -> Self {
        trace!("Creating new Storage instance");
        Self {
            api_clients: Vec::new(),
            chats: Vec::new(),
        }
    }

    pub fn add_api_client(&mut self, client: APIClient) {
        trace!("Adding new key to Storage");
        self.api_clients.push(client);
    }
    pub fn add_chat(&mut self, chat: Chat) {
        trace!("Adding new chat to Storage");
        self.chats.push(chat)
    }
    pub fn save(&self, filename: &Path) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Saving Storage to file: {}", filename.display());
        let encoded: Vec<u8> = bincode::serialize(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        trace!("File succesfully saved to {}", filename.display());
        Ok(())
    }

    pub fn load(filename: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        trace!("Loading Storage from file: {}", filename.display());
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let decoded: Self = bincode::deserialize(&buffer)?;
        trace!("File succesfully loaded from {}", filename.display());
        Ok(decoded)
    }
}
