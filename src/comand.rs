
pub fn message_otvet(chat_id:i64, text:&str, token:&str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        token,
    );
    let reved = serde_json::json!({
        "chat_id": chat_id,
        "text": text,
    });
    ureq::post(&url)
    .set("content-type","application/json")
    .send_string(&reved.to_string())?;

    

    
    Ok(())
}

pub fn comad_otvet1(text:&str) -> String {
    match text {
        "/start" => "bot startd...".to_string(),
        "/help" => "hello to comand: 
        /start,
        /help,
        /info,
        /author, ".to_string(),

        "/info" => "ето бот написан на чистом раст без всяких фреймворков и библиотек".to_string(),
        "/author" => "Автор бота: Виктор Дедка/@kitty_rec".to_string(),

        _ => format!("User message: ({})", text),
    }
}