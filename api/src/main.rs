// --- Axum Web Server for GitLab Productivity Metrics ---

mod gitlab_client;
mod metrics_engine;

use axum::serve;
use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use gitlab_client::GitLabClient;
use metrics_engine::{compute_group_metrics, compute_team_metrics, compute_user_metrics, get_all_users};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/groups/:group_id/metrics", get(get_group_metrics))
        .route("/teams/:team_id/metrics", get(get_team_metrics))
        .route("/users/:user_id/metrics", get(get_user_metrics))
        .route("/users", get(get_users));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct TimeRange {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Serialize)]
pub struct GroupMetrics {
    pub cycle_time_avg: String,
    pub deployment_frequency: String,
    pub total_commits: u32,
    pub inactive_projects: Vec<String>,
}

async fn get_group_metrics(
    Path(group_id): Path<String>,
    Query(params): Query<TimeRange>,
) -> Json<GroupMetrics> {
    let client = GitLabClient::new_from_env();
    let metrics = compute_group_metrics(&client, &group_id, &params).await;
    Json(metrics)
}

#[derive(Serialize)]
pub struct TeamMetrics {
    pub mr_throughput: u32,
    pub avg_review_time: String,
    pub workload_distribution: Vec<UserWorkload>,
}

#[derive(Serialize)]
pub struct UserWorkload {
    pub user: String,
    pub commits: u32,
    pub reviews: u32,
}

async fn get_team_metrics(
    Path(team_id): Path<String>,
    Query(params): Query<TimeRange>,
) -> Json<TeamMetrics> {
    let client = GitLabClient::new_from_env();
    let metrics = compute_team_metrics(&client, &team_id, &params).await;
    Json(metrics)
}

#[derive(Serialize)]
pub struct UserMetrics {
    pub commits_per_week: u32,
    pub mrs_created: u32,
    pub avg_mr_size: String,
    pub time_to_first_review_avg: String,
    pub rework_rate: String,
}

async fn get_user_metrics(
    Path(user_id): Path<String>,
    Query(params): Query<TimeRange>,
) -> Json<UserMetrics> {
    let client = GitLabClient::new_from_env();
    let metrics = compute_user_metrics(&client, &user_id, &params).await;
    Json(metrics)
}

#[derive(Serialize)]
pub struct Users {
    pub users: Vec<User>,
}

#[derive(Serialize)]
pub struct User {
    pub username: String,
}

async fn get_users() -> Json<Users> {
    let client = GitLabClient::new_from_env();
    let users = get_all_users(&client).await;
    Json(users)
}
