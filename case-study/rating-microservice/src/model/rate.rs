use validator::Validate;
use crate::schema::rates;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone)]
pub struct Rate {
    pub id: i32,
    pub value: i32,
    pub post_id: i32,
    pub author_id: i32
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RateDTO {
    #[validate(range(min = 1, max = 5))]
    pub value: i32,
    pub post_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "rates"]
pub struct NewRate {
    pub value: i32,
    pub post_id: i32,
    pub author_id: i32
}

impl Rate {
    pub fn new() -> Rate{
        Rate {
            id: -1,
            value: -1,
            post_id: -1,
            author_id: -1
        }
    }
}