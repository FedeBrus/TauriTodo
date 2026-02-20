use uuid::Uuid;
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

const STORE_PATH: &str = "tasks.json";

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
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
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    let new_task = Task::new(&msg);
    store.set(new_task.id.clone(), json!(new_task));
    app.emit("list-changed", "").unwrap();

    Ok(new_task)
}

#[tauri::command]
pub async fn get_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    let mut tasks: Vec<Task> = store.values()
        .into_iter()
        .filter_map(|val| serde_json::from_value(val).ok())
        .collect();
        
    tasks.sort_by_key(|t| t.msg.clone().to_lowercase());

    Ok(tasks)
}

#[tauri::command]
pub async fn delete_all(app: AppHandle) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    store.clear();
    app.emit("list-changed", "").unwrap();
    Ok(())
}

#[tauri::command]
pub async fn delete_task(app: AppHandle, id: String) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    store.delete(id);
    app.emit("list-changed", "").unwrap();
    Ok(())
}

#[tauri::command]
pub async fn mark_as_complete(app: AppHandle, id: String) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    if let Some(value) = store.get(&id) {

        let mut task: Task = serde_json::from_value(value).map_err(|e| e.to_string())?;

        if task.status == TaskStatus::TODO {
            task.status = TaskStatus::DONE;
        }        

        store.set(id, serde_json::to_value(task).map_err(|e| e.to_string())?);
        store.save().map_err(|e| e.to_string())?;
    }
    app.emit("list-changed", "").unwrap();
    Ok(())
}

#[tauri::command]
pub async fn mark_as_incomplete(app: AppHandle, id: String) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    if let Some(value) = store.get(&id) {

        let mut task: Task = serde_json::from_value(value).map_err(|e| e.to_string())?;

        if task.status == TaskStatus::DONE {
            task.status = TaskStatus::TODO;
        }        

        store.set(id, serde_json::to_value(task).map_err(|e| e.to_string())?);
        store.save().map_err(|e| e.to_string())?;
    }
    app.emit("list-changed", "").unwrap();
    Ok(())
}