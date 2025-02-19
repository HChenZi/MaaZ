use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{config::start_up::StartUpConfig, MaaError};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TaskRunningState {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskStatus {
    pub id: Option<i64>,
    #[serde(rename = "taskType")]
    pub task_type: TaskType,
    pub state: TaskRunningState,
}

impl From<TaskType> for TaskStatus {
    fn from(task_type: TaskType) -> Self {
        Self {
            id: None,
            task_type,
            state: TaskRunningState::Pending,
        }
    }
}

impl TryFrom<String> for TaskType {
    type Error = MaaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "StartUp" => Ok(TaskType::StartUp),
            _ => Err(MaaError::UnknowTaskError(value)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum TaskType {
    StartUp,
}

impl TaskType {
    pub fn get_string(self) -> String {
        match self {
            TaskType::StartUp => "start_up".to_owned(),
        }
    }
}

pub trait TaskParam: Serialize {
    fn get_param(&self) -> Value;
}

#[derive(Serialize)]
pub struct StartUpParam {
    pub package: String,
}

impl TaskParam for StartUpParam {
    fn get_param(&self) -> Value {
        #[allow(clippy::unwrap_used)]
        let inner = serde_json::to_value(self).unwrap();
        json!({
            "sub_start_app": inner,
        })
    }
}

impl From<StartUpConfig> for StartUpParam {
    fn from(config: StartUpConfig) -> Self {
        Self {
            package: config.client_type.get_package_name(),
        }
    }
}
