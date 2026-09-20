use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Chat {
    pub id: i64,
}
#[derive(Deserialize,Debug)]
pub struct Message {
    pub chat:Chat,
    pub text:Option<String>,
    pub message_id:i64,
}
#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id:i64,
    pub message:Option<Message>,
}
#[derive(Deserialize,Debug)]
pub struct Telegram {
    pub result: Vec<Update>,
}