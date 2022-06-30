use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsRoot {
    pub object: String,
    pub results: Vec<Project>,
    #[serde(rename = "next_cursor")]
    pub next_cursor: Value,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub page: Page,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub object: String,
    pub id: String,
    // #[serde(rename = "created_time")]
    // pub created_time: String,
    // #[serde(rename = "last_edited_time")]
    // pub last_edited_time: String,
    // #[serde(rename = "created_by")]
    // pub created_by: CreatedBy,
    // #[serde(rename = "last_edited_by")]
    // pub last_edited_by: LastEditedBy,
    // pub cover: Cover,
    // pub icon: Icon,
    // pub parent: Parent,
    // pub archived: bool,
    // pub properties: Properties,
    // pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub object: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LastEditedBy {
    pub object: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    #[serde(rename = "type")]
    pub type_field: String,
    pub external: External,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct External {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    #[serde(rename = "type")]
    pub type_field: String,
    pub emoji: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "database_id")]
    pub database_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "In stock")]
    pub in_stock: InStock,
    #[serde(rename = "Last ordered")]
    pub last_ordered: LastOrdered,
    #[serde(rename = "Price")]
    pub price: Price,
    #[serde(rename = "Recipes")]
    pub recipes: Recipes,
    #[serde(rename = "Cost of next trip")]
    pub cost_of_next_trip: CostOfNextTrip,
    #[serde(rename = "Name")]
    pub name: Name,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InStock {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub checkbox: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LastOrdered {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub date: Date,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    pub start: String,
    pub end: Value,
    #[serde(rename = "time_zone")]
    pub time_zone: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub number: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recipes {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub relation: Vec<Relation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CostOfNextTrip {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub formula: Formula,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Formula {
    #[serde(rename = "type")]
    pub type_field: String,
    pub number: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: Vec<Title>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text,
    pub annotations: Annotations,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    pub link: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {}
