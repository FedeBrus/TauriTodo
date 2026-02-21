use std::sync::Arc;

use uuid::Uuid;
use serde_json::json;
use tauri::{AppHandle, Emitter, Wry};
use tauri_plugin_store::{Store, StoreExt};
use chrono::prelude::*;

const STORE_PATH: &str = "tasks.json";

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
enum TaskStatus {
    DONE,
    TODO
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    id: String,
    text: String,
    status: TaskStatus,
    date: NaiveDate,
    expiration: Option<NaiveDate>
}

fn string_to_date_value(str: Option<&str>) -> Result<u32, String> {
    let number_string = str.ok_or("Date value is missing".to_string())?;
    number_string.parse::<u32>().map_err(|e| format!("Date format error: {}", e))
}

fn string_to_datetime(expiration_string: &str) -> Result<NaiveDate, String> {
    let mut split = expiration_string.split('-');
    let year = string_to_date_value(split.next())?;
    let month = string_to_date_value(split.next())?;
    let day = string_to_date_value(split.next())?;

    NaiveDate::from_ymd_opt(year as i32, month, day).ok_or("Invalid or absent expiration date".to_string())
}

impl Task {
    pub fn new(text: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            status: TaskStatus::TODO,
            date: Local::now().date_naive(),
            expiration: None 
        }
    }

    pub fn new_with_expiration(text: &str, expiration: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            status: TaskStatus::TODO,
            date: Local::now().date_naive(),
            expiration: string_to_datetime(&expiration).ok()
        }
    }

    pub fn complete(&mut self) {
        if self.status == TaskStatus::TODO {
            self.status = TaskStatus::DONE;
        }
    }

    pub fn incomplete(&mut self) {
        if self.status == TaskStatus::DONE {
            self.status = TaskStatus::TODO;
        }
    }

    pub fn update_text(&mut self, new_text: &str) {
        if !new_text.is_empty() {
            self.text = String::from(new_text);
        }
    }
}

fn get_store(app: &AppHandle) -> Result<Arc<Store<Wry>>, String> {
    app.store(STORE_PATH).map_err(|e| e.to_string())
}

fn emit_list_change(app: &AppHandle) {
    app.emit("list-changed", "").unwrap();
}

#[tauri::command]
pub async fn add_task(app: AppHandle, msg: String) -> Result<Task, String> {
    let store = get_store(&app)?;
    let new_task = Task::new(&msg);
    store.set(new_task.id.clone(), json!(new_task));
    emit_list_change(&app);
    Ok(new_task)
}

#[tauri::command]
pub async fn get_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let store = get_store(&app)?;
    let mut tasks: Vec<Task> = store.values()
        .into_iter()
        .filter_map(|val| serde_json::from_value(val).ok())
        .collect();
        
    tasks.sort_by_key(|t| t.text.clone().to_lowercase());

    Ok(tasks)
}

#[tauri::command]
pub async fn delete_all(app: AppHandle) -> Result<(), String> {
    let store = get_store(&app)?;
    store.clear();
    emit_list_change(&app);
    Ok(())
}

#[tauri::command]
pub async fn delete_task(app: AppHandle, id: String) -> Result<(), String> {
    let store = get_store(&app)?;
    store.delete(id);
    emit_list_change(&app);
    Ok(())
}

#[tauri::command]
pub async fn mark_as_complete(app: AppHandle, id: String) -> Result<(), String> {
    let store = get_store(&app)?;
    if let Some(value) = store.get(&id) {

        let mut task: Task = serde_json::from_value(value).map_err(|e| e.to_string())?;

        task.complete();

        store.set(id, serde_json::to_value(task).map_err(|e| e.to_string())?);
        store.save().map_err(|e| e.to_string())?;
    }
    emit_list_change(&app);
    Ok(())
}

#[tauri::command]
pub async fn mark_as_incomplete(app: AppHandle, id: String) -> Result<(), String> {
    let store = get_store(&app)?;
    if let Some(value) = store.get(&id) {

        let mut task: Task = serde_json::from_value(value).map_err(|e| e.to_string())?;
        
        task.incomplete();

        store.set(id, serde_json::to_value(task).map_err(|e| e.to_string())?);
        store.save().map_err(|e| e.to_string())?;
    }
    emit_list_change(&app);
    Ok(())
}

#[tauri::command]
pub async fn edit_task_text(app: AppHandle, id: String, msg: String) -> Result<(), String> {
    let store = get_store(&app)?;

    if let Some(value) = store.get(&id) {
        let mut task: Task = serde_json::from_value(value).map_err(|e| e.to_string())?;

        task.update_text(&msg);

        store.set(id, serde_json::to_value(task).map_err(|e| e.to_string())?);
        store.save().map_err(|e| e.to_string())?;
    }

    emit_list_change(&app);
    Ok(())
}