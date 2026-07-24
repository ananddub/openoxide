use auto_di::resolve;
use minijinja::{Environment, Value, context};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

use crate::repository::{
    ApplicationRepository, ComposeProjectRepository, EnvironmentRepository, ProjectRepository,
};

pub fn generate_env_app(
    db_env: String,
    db_project: String,
    db_app: String,
) -> BTreeMap<String, String> {
    let map: HashMap<String, String> = convert_env_to_map(&db_env);
    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project,
    );
    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);
    let app_env = fix_env(env_project_map(map, pro_map), &db_app);
    convert_env_to_btree_map(&app_env)
}

pub async fn generate_env_app_with_id(
    env_id: i64,
    pr_id: i64,
    app_id: i64,
) -> Option<HashMap<String, String>> {
    let repo_env = resolve::<EnvironmentRepository>().await.ok()?;
    let repo_project = resolve::<ProjectRepository>().await.ok()?;
    let repo_app = resolve::<ApplicationRepository>().await.ok()?;

    let db_env = repo_env.get_by_id(env_id).await.ok()??;
    let db_project_env = repo_project.get_by_id(pr_id).await.ok()??;
    let db_app_env = repo_app.get_by_id(app_id).await.ok()??;

    let map: HashMap<String, String> = convert_env_to_map(&db_env.env_var);

    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project_env.env_var,
    );

    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);

    let app_env = fix_env(
        env_project_map(map, pro_map),
        &db_app_env.env_var.unwrap_or_default(),
    );

    Some(convert_env_to_map(&app_env))
}

pub async fn generate_env_compose(
    env_id: i64,
    pr_id: i64,
    compose_id: i64,
) -> Option<HashMap<String, String>> {
    let repo_env = resolve::<EnvironmentRepository>().await.ok()?;
    let repo_project = resolve::<ProjectRepository>().await.ok()?;
    let repo_compose = resolve::<ComposeProjectRepository>().await.ok()?;

    let db_env = repo_env.get_by_id(env_id).await.ok()??;
    let db_project_env = repo_project.get_by_id(pr_id).await.ok()??;
    let db_compose_env = repo_compose.get_by_id(compose_id).await.ok()??;
    let map: HashMap<String, String> = convert_env_to_map(&db_env.env_var);

    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project_env.env_var,
    );

    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);

    let compose_env = fix_env(
        env_project_map(map, pro_map),
        &db_compose_env.env_var.unwrap_or_default(),
    );

    Some(convert_env_to_map(&compose_env))
}

pub async fn generate_env_db(env_id: i64, db_env_var: &str) -> Option<HashMap<String, String>> {
    let repo_env = resolve::<EnvironmentRepository>().await.ok()?;
    let repo_project = resolve::<ProjectRepository>().await.ok()?;

    let db_env = repo_env.get_by_id(env_id).await.ok()??;
    let db_project_env = repo_project.get_by_id(db_env.project_id).await.ok()??;
    let map: HashMap<String, String> = convert_env_to_map(&db_env.env_var);

    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project_env.env_var,
    );

    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);

    let resolved_db_env = fix_env(env_project_map(map, pro_map), db_env_var);

    Some(convert_env_to_map(&resolved_db_env))
}

fn convert_env_to_map(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let value = parts.next()?.trim().to_string();
            Some((key, value))
        })
        .collect()
}

fn convert_env_to_btree_map(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let value = parts.next()?.trim().to_string();
            Some((key, value))
        })
        .collect()
}

fn env_project_map(env: HashMap<String, String>, proj: HashMap<String, String>) -> Value {
    context! {
        enviroment => env,
        ENVIROMENT => env,
        PROJECT => proj,
        project => proj
    }
}

fn fix_env<S: Serialize>(map: S, text: &str) -> String {
    let mut env = Environment::new();
    env.add_template("env", &text).unwrap();
    let template = env.get_template("env").unwrap();
    template.render(map).unwrap()
}
