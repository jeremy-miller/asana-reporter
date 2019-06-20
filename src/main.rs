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
    for project in project_response.data.into_iter() {
        let yesterday = Utc::now() - Duration::days(1);
        let yesterday = yesterday.format("%Y-%m-%dT%H:%M:%S");
        let task_url = format!("https://app.asana.com/api/1.0/projects/{}/tasks?opt_fields=name,completed,completed_at&completed_since={}", project.gid, yesterday);
        let task_response: TaskResponse = client
            .get(&task_url)
            .header("Authorization", bearer_header_value.clone())
            .send()?
            .json()?;
        println!("{:?}", task_response);
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
