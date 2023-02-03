use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Habit {
    key: String,
    name: String,
    #[serde(rename = "userId")]
    user_id: String,
}
