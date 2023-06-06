#[derive(Clone, Debug)]
pub struct TodoItem {
    pub id: u64,
    pub text: String,
    pub done: bool,
}