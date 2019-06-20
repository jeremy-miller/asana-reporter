use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct AsanaResponse {
    data: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
struct Entry {
    gid: String,
}

#[derive(Deserialize, Debug)]
struct TaskResponse {
    data: Vec<Task>,
}

#[derive(Deserialize, Debug)]
struct Task {
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    name: String,
}

fn main() -> Result<(), reqwest::Error> {
    let access_token = get_access_token();
    let bearer_header_value = format!("Bearer {}", access_token);
    let client = reqwest::Client::new();
    let workspace_url = "https://app.asana.com/api/1.0/workspaces";
    let workspace_response: AsanaResponse = client
        .get(workspace_url)
        .header("Authorization", bearer_header_value.clone())
        .send()?
        .json()?;
    let project_url = format!(
        "https://app.asana.com/api/1.0/workspaces/{}/projects",
        workspace_response.data[0].gid
    );
    let project_response: AsanaResponse = client
        .get(&project_url)
        .header("Authorization", bearer_header_value.clone())
        .send()?
        .json()?;
    let mut all_completed_tasks: Vec<Task> = Vec::new();
    for project in project_response.data.into_iter() {
        let completed_since = Utc::now() - Duration::days(1);
        let completed_since = completed_since.format("%Y-%m-%dT%H:%M:%S");
        let task_url = format!("https://app.asana.com/api/1.0/projects/{}/tasks?opt_fields=name,completed,completed_at&completed_since={}", project.gid, completed_since);
        let task_response: TaskResponse = client
            .get(&task_url)
            .header("Authorization", bearer_header_value.clone())
            .send()?
            .json()?;
        let completed_tasks: Vec<Task> = task_response
            .data
            .into_iter()
            .filter(|task| task.completed)
            .collect();
        all_completed_tasks.extend(completed_tasks);
    }
    all_completed_tasks.sort_by_key(|task| task.completed_at);
    for task in all_completed_tasks {
        println!("- {}", task.name);
    }
    Ok(())
}

fn get_access_token() -> String {
    let key = "ASANA_PERSONAL_ACCESS_TOKEN";
    match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("Environment variable {} not found: {}", key, e),
    }
}
