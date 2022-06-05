use reqwest;
use reqwest::header::HeaderValue;
use reqwest::header::{self, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};

#[tokio::main]
pub async fn get_pages(
    token: &String,
    database_id: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = notion_client(&token);
    let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);
    let res = client.unwrap().post(url).send().await?;
    let status = res.status();
    println!("status = {:?}", status);
    let body = res.text().await?;

    Ok(body)
}

fn notion_client(token: &String) -> Result<Client, Error> {
    let token = format!("Bearer {secret}", secret = token);
    let mut headers = header::HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, token.parse().unwrap());
    headers.insert("Notion-Version", "2022-02-22".parse().unwrap());

    Client::builder().default_headers(headers).build()
}

// TODO: implement maybe later...
// pub struct NotionAPIClient {
//     client: ClientBuilder
// }
//
// impl NotionAPIClient {
//     fn new(&mut self, token: String) -> Result<NotionAPIClient, E> {
//         Ok(NotionAPIClient { })
//     }
// }
