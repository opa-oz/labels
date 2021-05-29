use reqwest::header::{ACCEPT, USER_AGENT};

use std::{env, error::Error, time::Duration};

use crate::label::Label;

const AUTH_HEADER: &str = "x-oauth-basic";

fn get_token<'a>() -> Result<String, &'a str> {
    let token = env::var("LABELS_TOKEN").unwrap_or_default();
    if token.is_empty() {
        return Err("Token not found");
    }
    Ok(token)
}

fn labels(owner: &str, repo: &str) -> Result<Vec<Label>, Box<dyn Error>> {
    let token = match get_token() {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let timeout = Duration::new(5, 0);
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/labels",
        owner = owner,
        repo = repo,
    );

    let response = reqwest::blocking::Client::new()
        .get(request_url)
        .timeout(timeout)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "labels")
        .send()?;

    if !response.status().is_success() {
        panic!("Error: status code {}", response.status());
    }

    let labels: Vec<Label> = response.json()?;

    Ok(labels)
}

pub(crate) fn print_labels(owner: &str, repo: &str) -> Result<(), Box<dyn Error>> {
    let labels = labels(owner, repo)?;
    let pretty = serde_json::to_string_pretty(&labels)?;
    println!("{}", pretty);

    Ok(())
}

pub(crate) fn update_labels(owner: &str, repo: &str) -> Result<(), Box<dyn Error>> {
    todo!();
}
