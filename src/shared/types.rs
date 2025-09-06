#[derive(Debug, Clone, PartialEq)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

// Layout mode enum
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutMode {
    Public,
    Authenticated,
}
