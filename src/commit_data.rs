pub use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub tree: String,           // Hash of tree
    pub parent: Option<String>, // Hash of parent commit
    pub author: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

trait DisplayCommit {
    fn format(&self) -> String;
}

impl DisplayCommit for Commit {
    fn format(&self) -> String {
        format!(
            "Commit {}\n Author {}\n Date {}\n\n {}",
            "HASH_PLACEHOLDER", self.author, self.timestamp, self.message
        )
    }
}
