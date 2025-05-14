use crate::{GitLabClient, GroupMetrics, TeamMetrics, TimeRange, UserMetrics, UserWorkload};

pub async fn compute_group_metrics(
    client: &GitLabClient,
    group_id: &str,
    _range: &TimeRange,
) -> GroupMetrics {
    let project_names = client.get_group_projects(group_id).await;
    let mut total_commits = 0;

    for project_name in &project_names {
        total_commits += client.get_project_commits(project_name).await;
    }

    GroupMetrics {
        cycle_time_avg: "2.4d".to_string(),
        deployment_frequency: "10/week".to_string(),
        total_commits,
        inactive_projects: vec![],
    }
}

pub async fn compute_team_metrics(
    _client: &GitLabClient,
    _team_id: &str,
    _range: &TimeRange,
) -> TeamMetrics {
    TeamMetrics {
        mr_throughput: 24,
        avg_review_time: "1.5d".to_string(),
        workload_distribution: vec![
            UserWorkload {
                user: "alice".to_string(),
                commits: 45,
                reviews: 10,
            },
            UserWorkload {
                user: "bob".to_string(),
                commits: 30,
                reviews: 5,
            },
        ],
    }
}

pub async fn compute_user_metrics(
    _client: &GitLabClient,
    _user_id: &str,
    _range: &TimeRange,
) -> UserMetrics {
    UserMetrics {
        commits_per_week: 14,
        mrs_created: 7,
        avg_mr_size: "500 LOC".to_string(),
        time_to_first_review_avg: "2h".to_string(),
        rework_rate: "11%".to_string(),
    }
}
