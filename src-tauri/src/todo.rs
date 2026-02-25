use std::sync::Arc;

use chrono::prelude::*;
use serde_json::json;
use tauri::{AppHandle, Emitter, Wry};
use tauri_plugin_store::{Store, StoreExt};
use uuid::Uuid;

const STORE_PATH: &str = "tasks.json";

#[derive(
    Debug, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd, Eq, Ord, Copy, Clone,
)]
enum TaskStatus {
    Done,
    Todo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SortingMethod {
    Text,
    Tag,
    Date,
    Expiration,
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd, Eq, Ord, Copy, Clone,
)]
enum TaskTag {
    NoTag,
    Work,
    Study,
    Hobby,
    Housework,
    Shopping,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    id: String,
    text: String,
    status: TaskStatus,
    date: NaiveDate,
    expiration: NaiveDate,
    tag: TaskTag,
}

fn string_to_date_value(str: Option<&str>) -> Result<u32, String> {
    let number_string = str.ok_or("Date value is missing".to_string())?;
    number_string
        .parse::<u32>()
        .map_err(|e| format!("Date format error: {}", e))
}

fn string_to_date(expiration_string: &str) -> Result<NaiveDate, String> {
    let mut split = expiration_string.split('-');
    let year = string_to_date_value(split.next())?;
    let month = string_to_date_value(split.next())?;
    let day = string_to_date_value(split.next())?;

    NaiveDate::from_ymd_opt(year as i32, month, day)
        .ok_or("Invalid or absent expiration date".to_string())
}

impl Task {
    pub fn new(text: &str, expiration: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            status: TaskStatus::Todo,
            date: Local::now().date_naive(),
            expiration: string_to_date(&expiration)
                .ok()
                .unwrap_or(Local::now().date_naive()),
            tag: TaskTag::NoTag,
        }
    }

    pub fn complete(&mut self) {
        if self.status == TaskStatus::Todo {
            self.status = TaskStatus::Done;
        }
    }

    pub fn incomplete(&mut self) {
        if self.status == TaskStatus::Done {
            self.status = TaskStatus::Todo;
        }
    }

    pub fn update_text(&mut self, new_text: &str) {
        if !new_text.is_empty() {
            self.text = String::from(new_text);
        }
    }

    pub fn update_expiration(&mut self, new_expiration: &str) {
        if let Some(naive_date) = string_to_date(new_expiration).ok() {
            self.expiration = naive_date;
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
pub async fn add_task(app: AppHandle, text: String, expiration: String) -> Result<Task, String> {
    let store = get_store(&app)?;
    let new_task = Task::new(&text, &expiration);
    store.set(new_task.id.clone(), json!(new_task));
    emit_list_change(&app);
    Ok(new_task)
}

#[tauri::command]
pub async fn get_tasks(
    app: AppHandle,
    sorting_method: SortingMethod,
    ascending: bool,
) -> Result<Vec<Task>, String> {
    let store = get_store(&app)?;
    let mut tasks: Vec<Task> = store
        .values()
        .into_iter()
        .filter_map(|val| serde_json::from_value(val).ok())
        .collect();

    sort_tasks_by(&mut tasks, sorting_method, ascending);

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
    }
    emit_list_change(&app);
    Ok(())
}

#[tauri::command]
pub async fn edit_task(
    app: AppHandle,
    id: String,
    text: String,
    expiration: String,
) -> Result<(), String> {
    let store = get_store(&app)?;

    if let Some(value) = store.get(&id) {
        let mut task: Task = serde_json::from_value(value).map_err(|e| e.to_string())?;

        task.update_text(&text);
        task.update_expiration(&expiration);

        store.set(id, serde_json::to_value(task).map_err(|e| e.to_string())?);
        store.save().map_err(|e| e.to_string())?;
    }

    emit_list_change(&app);
    Ok(())
}

fn sort_tasks_by(tasks: &mut Vec<Task>, sorting_method: SortingMethod, ascending: bool) {
    match sorting_method {
        SortingMethod::Text => tasks.sort_by_key(|t| (t.status, t.text.clone())),
        SortingMethod::Date => tasks.sort_by_key(|t| (t.status, t.date)),
        SortingMethod::Expiration => tasks.sort_by_key(|t| (t.status, t.expiration)),
        SortingMethod::Tag => tasks.sort_by_key(|t| (t.status, t.tag)),
    }

    if !ascending {
        tasks.reverse();
    }
}
