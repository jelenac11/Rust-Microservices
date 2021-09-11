use std::fmt;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "ROLE_ADMIN" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "ROLE_USER"),
            Role::Admin => write!(f, "ROLE_ADMIN"),
        }
    }
}