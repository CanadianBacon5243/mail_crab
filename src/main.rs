use clap::Parser;
use iced::Application;

pub fn main() -> iced::Result {
    Mail::run(iced::Settings::default())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Username to log in
    #[arg(short, long)]
    user: String,

    /// Password to log in
    #[arg(short, long)]
    pass: String,

    /// Imap server to fetch mail from
    #[arg(short, long)]
    server: String,

    /// Which email to fetch
    #[arg(short, long)]
    index: u32
}

struct Mail {
    body: String
}

#[derive(Debug, Clone, Copy)]
enum Message {

}

impl iced::Sandbox for Mail {
    type Message = Message;
    fn new() -> Self {
        let args = Args::parse();
        Self {
            body: fetch_inbox_one_item(&args.user, &args.pass, &args.server, args.index).unwrap().unwrap()
        }
    }
    fn title(&self) -> String {
        String::from("Mail Crab")
    }
    fn update(&mut self, _message: Message) {

    }
    fn view(&self) -> iced::Element<Message> {
        iced::widget::column![iced::widget::text(self.body.clone())].into()
    }
}

//based off the example of the imap documentation
fn fetch_inbox_one_item(username : &str,
                        password : &str,
                        domain : &str,
                        index : u32) -> imap::error::Result<Option<String>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();
    let mut imap_session = client.login(username, password).map_err(|e| e.0)?;

    imap_session.select("INBOX")?;

    let messages = imap_session.fetch(index.to_string(), "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    let body = message.body().expect("Message did not have a body");
    let body = std::str::from_utf8(body).expect("message not valid utf-8").to_string();

    imap_session.logout()?;

    Ok(Some(body))
}
