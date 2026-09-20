
mod comand; 
mod struct1; 
use struct1::Telegram;
use comand::message_otvet;
use comand::comad_otvet1;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "8885004391:AAHTbiI1EOyOmayh2bMnw-F4XflmRTa8UZo";
    let mut offset =0;
    println!("bot start {}>.....", offset);

    loop {
    let url = format!(
        "https://api.telegram.org/bot{}/getUpdates?offset={}&timeout=30",
        token, offset,
    );
    let responese = ureq::get(&url)
    .call()?
    .into_string()?;

    let parsed:Telegram = serde_json::from_str(&responese)?;

    for update in parsed.result {
        offset = update.update_id + 1;
        if let Some(msg) = &update.message {
            if let Some(txt) = &msg.text {
                println!("{}", txt);
                let answer = comad_otvet1(txt);
                
                message_otvet(msg.chat.id, &answer, token)?;
               

            }
        }
    }
 
 }


    Ok(())
}
