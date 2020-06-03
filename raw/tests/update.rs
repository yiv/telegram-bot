use std::fs::File;
use std::io::prelude::*;

use telegram_bot_raw::types::message::MessageKind;
use telegram_bot_raw::types::update::{Update, UpdateKind};
use telegram_bot_raw::{SendMessage, GroupId};
use telegram_bot_raw::Chat;
use telegram_bot_raw::Group;

macro_rules! make_test {
    ($asset: ident, $test: expr) => {
        #[test]
        fn $asset() {
            let data = {
                let filename = format!("tests/update_assets/{}.json", stringify!($asset));
                let mut data = Vec::new();
                let mut file = File::open(filename).unwrap();
                file.read_to_end(&mut data).unwrap();
                data
            };
            let update = serde_json::from_slice::<Update>(&data).unwrap();
            $test(update)
        }
    };
}

make_test!(migrate_from_chat_id, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateFromChatId { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});

make_test!(migrate_to_chat_id, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateToChatId { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});

make_test!(inline_query, |update: Update| {
    if let UpdateKind::InlineQuery(_query) = update.kind {
        return ();
    }

    assert!(false)
});

#[test]
fn test_encode_update(){
    let s = r#"{"update_id":693714082,"message":{"message_id":146,"from":{"id":1022260800,"is_bot":false,"first_name":"barce","last_name":"shao","language_code":"zh-hans"},"chat":{"id":1022260800,"first_name":"barce","last_name":"shao","type":"private"},"date":1589885162,"forward_from":{"id":84210004,"is_bot":true,"first_name":"PollBot","username":"PollBot"},"forward_date":1589874674,"text":"Let's create a new poll. First, send me the question."}}"#;
    let foward_from_channel = r#"{"update_id":693714083,"message":{"message_id":149,"from":{"id":1022260800,"is_bot":false,"first_name":"barce","last_name":"shao","language_code":"zh-hans"},"chat":{"id":1022260800,"first_name":"barce","last_name":"shao","type":"private"},"date":1589957567,"forward_from_chat":{"id":-1001337931577,"title":"TestChannel","username":"barceshao","type":"channel"},"forward_from_message_id":71,"forward_date":1589354052,"photo":[{"file_id":"AgACAgUAAxkBAAOVXsTTvzqLgkH1L_4HZYZq6UoiDnsAArKqMRshbNlVxWJn6YTOC-Vtc8NqdAADAQADAgADbQADveoBAAEZBA","file_unique_id":"AQADbXPDanQAA73qAQAB","file_size":13230,"width":240,"height":320},{"file_id":"AgACAgUAAxkBAAOVXsTTvzqLgkH1L_4HZYZq6UoiDnsAArKqMRshbNlVxWJn6YTOC-Vtc8NqdAADAQADAgADeAADvuoBAAEZBA","file_unique_id":"AQADbXPDanQAA77qAQAB","file_size":88369,"width":599,"height":800},{"file_id":"AgACAgUAAxkBAAOVXsTTvzqLgkH1L_4HZYZq6UoiDnsAArKqMRshbNlVxWJn6YTOC-Vtc8NqdAADAQADAgADeQADv-oBAAEZBA","file_unique_id":"AQADbXPDanQAA7_qAQAB","file_size":239430,"width":956,"height":1276}]}}"#;
    let send_a_photo = r#"{"update_id":693714086,"message":{"message_id":154,"from":{"id":1022260800,"is_bot":false,"first_name":"barce","last_name":"shao","language_code":"zh-hans"},"chat":{"id":1022260800,"first_name":"barce","last_name":"shao","type":"private"},"date":1589958572,"photo":[{"file_id":"AgACAgUAAxkBAAOaXsTXrJpeG4gP_tUlUQXSDA14lnkAAkypMRvgnylWGv_0CCWYhGvWX2ZqdAADAQADAgADbQADpzwEAAEZBA","file_unique_id":"AQAD1l9manQAA6c8BAAB","file_size":3352,"width":200,"height":200}],"caption":"2sf"}}"#;
    let edit_message = r#"{"update_id":693714088,"edited_message":{"message_id":155,"from":{"id":1022260800,"is_bot":false,"first_name":"barce","last_name":"shao","language_code":"zh-hans"},"chat":{"id":1022260800,"first_name":"barce","last_name":"shao","type":"private"},"date":1589958752,"edit_date":1589958770,"text":"456"}}"#;
    let u = serde_json::from_str::<Update>(s).unwrap();
    println!("{:?}", u);

    let s = serde_json::to_string(&u).unwrap();
    println!("{}", s);

}

#[test]
fn test_decode_send_message(){
    let g = Chat::Group(Group::default());
    let m = SendMessage::new(g, "hel");
    let s = serde_json::to_string(&m).unwrap();
    println!("{}", s);

    let s = r#"{"chat_id":550,"text":"hel", "parse_mode":"html", "disable_web_page_preview":true, "disable_notification":true, "reply_to_message_id":true}"#;
    let m = serde_json::from_str::<SendMessage>(s).unwrap();
    println!("{:?}", m);
}
