use uuid::Uuid;
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum TaskStatus {
    DONE,
    TODO
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    id: String,
    msg: String,
    status: TaskStatus
}

impl Task {
    pub fn new(msg: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            msg: msg.to_string(),
            status: TaskStatus::TODO
        }
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::DONE
    }
}


#[tauri::command]
pub async fn add_task(app: AppHandle, msg: String) -> Result<Task, String> {
    let store = app.store("kanban.json").map_err(|e| e.to_string())?;
    let new_task = Task::new(&msg);
    store.set(new_task.id.clone(), json!(new_task));
    app.emit("list-changed", "").unwrap();

    Ok(new_task)
}

#[tauri::command]
pub async fn get_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let store = app.store("kanban.json").map_err(|e| e.to_string())?;
    let tasks: Vec<Task> = store.values()
        .into_iter()
        .filter_map(|val| serde_json::from_value(val).ok())
        .collect();

    Ok(tasks)
}

#[tauri::command]
pub async fn delete_all(app: AppHandle) -> Result<(), String> {
    let store = app.store("kanban.json").map_err(|e| e.to_string())?;
    store.clear();
    app.emit("list-changed", "").unwrap();
    Ok(())
}

