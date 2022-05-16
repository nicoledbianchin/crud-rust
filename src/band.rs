#[derive(Serialize, Deserialize)]
pub struct Band{
    pub id: Option<i32>,
    pub name: String,
    pub genre: String,
    pub hometown: String,
    pub age: i32
}