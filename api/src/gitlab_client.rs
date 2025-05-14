use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::env;

pub struct GitLabClient {
    base_url: String,
    private_token: String,
    http: Client,
}

#[derive(Deserialize)]
struct Project {
    id: u64,
    name: String,
    archived: bool,
}

static _DOTENV: Lazy<()> = Lazy::new(|| {
    dotenvy::dotenv().ok();
});

impl GitLabClient {
    pub fn new_from_env() -> Self {
        Lazy::force(&_DOTENV);

        let base_url = env::var("GITLAB_BASE_URL").expect("GITLAB_BASE_URL must be set");
        let private_token = env::var("GITLAB_TOKEN").expect("GITLAB_PRIVATE_TOKEN must be set");

        GitLabClient {
            base_url,
            private_token,
            http: Client::new(),
        }
    }

    pub async fn get_group_projects(&self, group_id: &str) -> Vec<String> {
        let url = format!("{}/api/v4/groups/{}/projects", self.base_url, group_id);
        let response = self
            .http
            .get(url)
            .header("PRIVATE-TOKEN", &self.private_token)
            .send()
            .await;

        match response {
            Ok(resp) => match resp.json::<Vec<Project>>().await {
                Ok(projects) => projects.into_iter().map(|p| p.name).collect(),
                Err(_) => vec![],
            },
            Err(_) => vec![],
        }
    }

    pub async fn get_project_commits(&self, project_id: &str) -> u32 {
        let url = format!(
            "{}/api/v4/projects/{}/repository/commits",
            self.base_url, project_id
        );
        let response = self
            .http
            .get(url)
            .header("PRIVATE-TOKEN", &self.private_token)
            .query(&[("per_page", "1")]) // Minimize payload
            .send()
            .await;

        match response {
            Ok(resp) => match resp.headers().get("X-Total") {
                Some(val) => val.to_str().unwrap_or("0").parse().unwrap_or(0),
                None => 0,
            },
            Err(_) => 0,
        }
    }

    pub async fn get_merge_requests(&self, project_id: &str) -> u32 {
        let url = format!(
            "{}/api/v4/projects/{}/merge_requests",
            self.base_url, project_id
        );
        let response = self
            .http
            .get(url)
            .header("PRIVATE-TOKEN", &self.private_token)
            .query(&[("per_page", "1")])
            .send()
            .await;

        match response {
            Ok(resp) => match resp.headers().get("X-Total") {
                Some(val) => val.to_str().unwrap_or("0").parse().unwrap_or(0),
                None => 0,
            },
            Err(_) => 0,
        }
    }
}
