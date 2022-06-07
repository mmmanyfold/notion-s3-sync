use reqwest;
use reqwest::header::{self, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Error};
use crate::repo::notion::types;
use crate::repo::notion::types::Ip;

pub async fn get_pages(
    token: &String,
    database_id: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = notion_client(&token);
    // let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);

    let json: Ip = reqwest::get("http://httpbin.org/ip").await?.json().await?;
    // let response = client.unwrap().post(url).send();
    // let status = response.status();
    // println!("status = {}", status);

    // let root = response.await?.json::<types::Root>().await?;
    // let root: Value = response.json::<serde_json::Value>().await?;
    println!("json = {:?}", json.origin);

    Ok(())
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
