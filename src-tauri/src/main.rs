// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;
// reference:
// https://gist.github.com/captainhusaynpenguin/5bdb6fcb141628b6865619bcd1c827fd
use std::sync::{Once, Mutex};
use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use app::{*};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            all_tags,
            add_new_tag,
            modify_tag,
            delete_tag,
            all_tasks,
            add_new_task,
            modify_task,
            delete_task,
            finish_task,
            unfinish_task,
            filter_tasks,
        ])
        .manage(AppState {db: Mutex::new(None) } )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct AppState {
    db: Mutex<Option<Db>>,
}
static SETUP: Once = Once::new();
fn setup_state(app_handle: &AppHandle, state: &AppState) {
    SETUP.call_once(|| {
        let mut option_db = state.db.lock().unwrap();
        *option_db = Some(Db::connect(util::get_database_file(app_handle))
            .expect("Unable to connect to application database.")
        );
    });
}

#[derive(Serialize)]
struct AppError {
    message: String
}
impl From<DbError> for AppError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::RusqliteError { error } => AppError {
                message: format!("Rusqlite error: {}", error.to_string())
            },
            DbError::TagDoesNotExistError { id } => AppError {
                message: format!("Tag {id} does not exist")
            },
            DbError::TaskDoesNotExistError { id } => AppError {
                message: format!("Task {id} does not exist")
            },
            DbError::TaskStatusError { id, actual_status } => AppError {
                message: if actual_status {
                    format!("Task {id} is already done")
                } else {
                    format!("Task {id} is already not done")
                }
            },
        }
    }
}
type AppResult<T> = Result<T, AppError>;

mod util {
    use std::path::PathBuf;
    use tauri::{AppHandle};

    pub fn is_dev(app_handle: &AppHandle) -> bool {
        return app_handle.path_resolver().app_data_dir().unwrap().to_str().unwrap().contains("com.tauri.dev");
    }

    pub fn get_data_directory(app_handle: &AppHandle) -> PathBuf {
        if is_dev(app_handle) {
            PathBuf::from("../dev-outputs")
        } else {
            app_handle.path_resolver().app_data_dir().unwrap()
        }
    }

    pub fn get_database_file(app_handle: &AppHandle) -> PathBuf {
        get_data_directory(app_handle).join("db.sqlite")
    }
}

#[tauri::command]
fn all_tags(app_handle: AppHandle, state: State<AppState>) -> AppResult<Vec<Tag>> {
    setup_state(&app_handle, state.deref());
    let binding = state.db.lock().unwrap();
    let db = binding.as_ref().unwrap();
    Ok(db.all_tags()?)
}

#[tauri::command]
fn add_new_tag(app_handle: AppHandle, state: State<AppState>, data: EditableTagData)
    -> AppResult<GeneratedTagData> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.add_new_tag(&data)?)
}

#[tauri::command]
fn modify_tag(app_handle: AppHandle, state: State<AppState>, id: TagId, data: EditableTagData)
               -> AppResult<()> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.modify_tag(id, &data)?)
}

#[tauri::command]
fn delete_tag(app_handle: AppHandle, state: State<AppState>, id: TagId)
              -> AppResult<()> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.delete_tag(id)?)
}

#[tauri::command]
fn all_tasks(app_handle: AppHandle, state: State<AppState>) -> AppResult<Vec<Task>> {
    setup_state(&app_handle, state.deref());
    let binding = state.db.lock().unwrap();
    let db = binding.as_ref().unwrap();
    Ok(db.all_tasks()?)
}

#[tauri::command]
fn add_new_task(app_handle: AppHandle, state: State<AppState>, data: EditableTaskData)
               -> AppResult<GeneratedTaskData> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.add_new_task(&data)?)
}

#[tauri::command]
fn modify_task(app_handle: AppHandle, state: State<AppState>, id: TaskId, data: EditableTaskData)
              -> AppResult<ModifiedTaskData> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.modify_task(id, &data)?)
}

#[tauri::command]
fn delete_task(app_handle: AppHandle, state: State<AppState>, id: TaskId)
              -> AppResult<()> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.delete_task(id)?)
}

#[tauri::command]
fn finish_task(app_handle: AppHandle, state: State<AppState>, id: TaskId)
               -> AppResult<FinishedTaskData> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.finish_task(id)?)
}

#[tauri::command]
fn unfinish_task(app_handle: AppHandle, state: State<AppState>, id: TaskId)
               -> AppResult<FinishedTaskData> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.unfinish_task(id)?)
}

#[tauri::command]
fn filter_tasks(app_handle: AppHandle, state: State<AppState>, filter: TaskFilterOptions)
               -> AppResult<Vec<Task>> {
    setup_state(&app_handle, state.deref());
    let mut binding = state.db.lock().unwrap();
    let db = binding.as_mut().unwrap();
    Ok(db.filter_tasks(|task| filter.passes(task))?)
}