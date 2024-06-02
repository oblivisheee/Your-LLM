use crate::{api::APIClient, chat::Chat};
use bincode;
use log::{error, trace};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub path: PathBuf,
    pub api_clients: Vec<APIClient>,
    pub chats: Vec<Chat>,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        trace!("Creating new Storage instance");
        Self {
            path: path.as_ref().to_path_buf(),
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

    pub fn update_chat(&mut self, chat: Chat) {
        if let Some(index) = self.chats.iter().position(|c| c.id == chat.id) {
            self.chats[index] = chat;
        } else {
            self.chats.push(chat);
        }
    }

    pub fn save(&self, filename: &Path) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Saving Storage to file: {}", filename.display());
        let encoded: Vec<u8> = bincode::serialize(&self)?;
        let mut file = File::create(filename).map_err(|e| {
            error!("Failed to create file: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        file.write_all(&encoded).map_err(|e| {
            error!("Failed to write to file: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        trace!("File succesfully saved to {}", filename.display());
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Loading Storage from file: {}", self.path.display());
        let mut file = File::open(self.path.clone()).map_err(|e| {
            error!("Failed to open file: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| {
            error!("Failed to read file: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        let decoded: Self = bincode::deserialize(&buffer).map_err(|e| {
            error!("Failed to deserialize data: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        self.api_clients = decoded.api_clients;
        self.chats = decoded.chats;
        trace!("File succesfully loaded from {}", self.path.display());
        Ok(())
    }
}
