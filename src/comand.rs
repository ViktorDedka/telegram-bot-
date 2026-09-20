


pub fn message_otvet(chat_id:i64, text:&str, token:&str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        token,
    );
    Ok(())
} 