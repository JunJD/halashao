use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum FeedType {
    #[serde(rename = "homefeed_recommend")]
    Recommend,
    #[serde(rename = "homefeed.fashion_v3")]
    Fashion,
    #[serde(rename = "homefeed.food_v3")]
    Food,
    #[serde(rename = "homefeed.cosmetics_v3")]
    Cosmetics,
    #[serde(rename = "homefeed.movie_and_tv_v3")]
    Movie,
    #[serde(rename = "homefeed.career_v3")]
    Career,
    #[serde(rename = "homefeed.love_v3")]
    Emotion,
    #[serde(rename = "homefeed.household_product_v3")]
    House,
    #[serde(rename = "homefeed.gaming_v3")]
    Game,
    #[serde(rename = "homefeed.travel_v3")]
    Travel,
    #[serde(rename = "homefeed.fitness_v3")]
    Fitness,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NoteType {
    Normal,
    Video,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchSortType {
    General,
    #[serde(rename = "popularity_descending")]
    MostPopular,
    #[serde(rename = "time_descending")]
    Latest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchNoteType {
    All = 0,
    Video = 1,
    Image = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub note_id: String,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub note_type: String,
    pub user: Value,
    pub img_urls: Vec<String>,
    pub video_url: String,
    pub tag_list: Vec<String>,
    pub at_user_list: Vec<String>,
    pub collected_count: String,
    pub comment_count: String,
    pub liked_count: String,
    pub share_count: String,
    pub time: i64,
    pub last_update_time: i64,
}
