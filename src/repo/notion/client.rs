use crate::repo::notion::types::ProjectsRoot;
use reqwest::header::{self, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self, Client, Error, StatusCode};

pub struct NotionAPIClient {
    inner: Result<Client, Error>,
}

impl NotionAPIClient {
    pub fn new(token: &str) -> Result<NotionAPIClient, Error> {
        Ok(NotionAPIClient {
            inner: build_with_headers(&token),
        })
    }

    pub async fn get_pages(self, database_id: String) -> Result<ProjectsRoot, Error> {
        let client = self.inner.unwrap();
        let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);
        let response = client.post(url).send().await?;
        let projects_response = match response.status() {
            status_ok @ StatusCode::OK => {
                println!("status = {}", status_ok);
                response
            }
            _ => {
                panic!("error: {:?}", response.status());
            }
        };
        let projects: ProjectsRoot = projects_response.json().await?;
        Ok(projects)
    }
}

fn build_with_headers(token: &str) -> Result<Client, Error> {
    let token = format!("Bearer {secret}", secret = token);
    let mut headers = header::HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, token.parse().unwrap());
    headers.insert("Notion-Version", "2022-02-22".parse().unwrap());

    Client::builder().default_headers(headers).build()
}
