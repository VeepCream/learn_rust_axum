use serde::{Deserialize, Serialize};

use super::quest_statuses::QuestStatuses;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct  BoardCheckingFilter {
    pub name: Option<String>,
    pub status: Option<QuestStatuses>,
}
