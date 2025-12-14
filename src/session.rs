use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct SessionState {
    pub last_confirmed_date_time: DateTime<Utc>,
    pub confirmed_contexts: HashMap<String, bool>,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            last_confirmed_date_time: Utc::now(),
            confirmed_contexts: HashMap::new(),
        }
    }
}

pub fn load_sessions_state(pid: u32) -> SessionState {
    let file_path = get_session_file_path(pid);

    if !Path::new(&file_path).exists() {
        return SessionState::default();
    }

    let json_session = fs::read_to_string(file_path).expect("Failed to read session file");
    let session: SessionState =
        serde_json::from_str(&json_session).expect("Failed to deserialize session");

    session
}

pub fn update_session(session: &mut SessionState, current_context: String, parent_pid: u32) {
    session.last_confirmed_date_time = Utc::now();
    session.confirmed_contexts.insert(current_context, true);
    save_session_state(parent_pid, session);
}

fn save_session_state(pid: u32, session: &SessionState) {
    let file_path = get_session_file_path(pid);

    let json = serde_json::to_string(&session).expect("Failed serialization");

    fs::write(file_path, json).expect("Failed to write session to tmp file system");
}

fn get_session_file_path(pid: u32) -> String {
    Path::new(&env::temp_dir())
        .join(format!("kcwrap-{pid}.json"))
        .to_string_lossy()
        .to_string()
}
