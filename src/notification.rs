mod discord;
mod email;

pub fn send_notification(msg: &str) -> anyhow::Result<()> {
    match discord::Discord::send(msg) {
        Ok(()) => return Ok(()),
        Err(e) => eprintln!("{e:?}"),
    };

    email::Email::send(msg)
}
