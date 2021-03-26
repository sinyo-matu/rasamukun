use flate2::read::GzDecoder;
use serde::Deserialize;
use serde_json::Value;
use serenity::framework::standard::CommandError;
use std::io::prelude::*;
#[derive(Deserialize, Debug)]
pub struct SOFItem {
    pub tags: Value,
    pub owner: Value,
    pub link: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct SOFResBody {
    pub items: Vec<SOFItem>,
    pub has_more: bool,
    pub quota_max: u32,
    pub quota_remaining: u32,
}

pub struct SOFClient;

impl SOFClient {
    pub async fn get_en_questions(tag: &str) -> Result<SOFResBody, CommandError> {
        let url = format!("https://api.stackexchange.com/2.2/questions?page=1&pagesize=3&order=desc&sort=activity&tagged=rust;{tag}&site=stackoverflow&filter=!9YdnSM68i",tag=tag);
        let bytes = reqwest::get(url).await?.bytes().await?.to_vec();
        let mut decoded = GzDecoder::new(&bytes[..]);
        let mut text = String::new();
        decoded.read_to_string(&mut text)?;
        let res_body = serde_json::from_str::<SOFResBody>(&text).unwrap();

        Ok(res_body)
    }

    pub async fn get_ja_questions(tag: &str) -> Result<SOFResBody, CommandError> {
        let url = format!("https://api.stackexchange.com/2.2/questions?page=1&pagesize=3&order=desc&sort=activity&tagged=rust;{tag}&site=ja.stackoverflow&filter=!9YdnSM68i",tag=tag);
        let bytes = reqwest::get(url).await?.bytes().await?.to_vec();
        let mut decoded = GzDecoder::new(&bytes[..]);
        let mut text = String::new();
        decoded.read_to_string(&mut text)?;
        let res_body = serde_json::from_str::<SOFResBody>(&text)?;
        Ok(res_body)
    }
}

#[derive(Deserialize, Debug)]
pub struct QiitaResBody(pub Vec<QiitaItem>);

#[derive(Deserialize, Debug)]
pub struct QiitaItem {
    pub comments_count: Option<u32>,
    pub id: String,
    pub like_count: Option<u32>,
    pub reactions_count: Option<u32>,
    pub title: String,
    pub updated_at: String,
    #[serde(alias = "url")]
    pub link: String,
    pub page_views_count: Option<u32>,
}
pub struct QiitaClient;

impl QiitaClient {
    pub async fn get_posts() -> Result<QiitaResBody, CommandError> {
        let url = format!("https://qiita.com/api/v2/items?page=1&per_page=3&query=tag:rust");
        let text = reqwest::get(url).await?.text().await?;
        let res_body = serde_json::from_str::<QiitaResBody>(&text)?;
        Ok(res_body)
    }
}
