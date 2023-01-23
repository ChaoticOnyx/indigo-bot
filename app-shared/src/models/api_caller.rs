use super::Secret;

#[derive(Debug, Clone)]
pub enum ApiCaller {
    System,
    Token(Secret),
}
