pub struct Message {
    pub data: Data,
    pub metadata: Metadata,
}

pub struct Data {
    pub content: String,
}

pub struct Metadata {
    pub timestamp: u64,
    pub sender: String,
}

impl Message {
    pub fn new(content: String, timestamp: u64, sender: String) -> Self {
        Self {
            data: Data { content },
            metadata: Metadata { timestamp, sender },
        }
    }

    pub fn get_content(&self) -> &str {
        &self.data.content
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }
}
