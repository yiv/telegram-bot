use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    // let token = "1108955016:AAHxCGvc4nEgLWSuGd5QPeZWWlYbirxLFzw".to_string();
    let token = "106309348245901312".to_string();
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        // println!("edwin 18 {:?}", update.clone());
        match update.kind {
            UpdateKind::Message(message) => {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    // Print received text message to stdout.
                    println!("<{}>: {}", &message.from.first_name, data);

                    // Answer message with "Hi".
                    api.send(message.text_reply(format!(
                        "Hi, {}! You just wrote '{}'",
                        &message.from.first_name, data
                    ))).await?;
                }
            }
            UpdateKind::ChannelPost(message) => {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    println!("<{}>", data);
                    api.send(message.text_reply(format!("Hi, You just wrote '{}'", data))).await?;
                }
            }
            _ => {
                // Message::video_reply()
            }
        }
        // if let UpdateKind::Message(message) = update.kind {
        //     if let MessageKind::Text { ref data, .. } = message.kind {
        //         // Print received text message to stdout.
        //         println!("<{}>: {}", &message.from.first_name, data);
        //
        //         // Answer message with "Hi".
        //         // api.send(message.text_reply(format!(
        //         //     "Hi, {}! You just wrote '{}'",
        //         //     &message.from.first_name, data
        //         // )))
        //         // .await?;
        //     }
        // }
    }
    Ok(())
}
