use validator::Validate;
use crate::schema::users;

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable, Serialize, Deserialize, Validate)]
#[table_name = "users"]
pub struct UserDTO {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub firstname: String,
    #[validate(length(min = 1))]
    pub lastname: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub role: String,
}

