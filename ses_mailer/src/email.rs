use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest};


#[derive(PartialEq, Debug)]
pub struct Email {
    pub sender: String,
    pub recipients: Vec<String>,
    pub html_body: String,
    pub raw_text_body: String,
    pub subject: String,
}

const CHARSET: &str = &"UTF-8";

impl Into<SendEmailRequest> for &Email {
    fn into(self) -> SendEmailRequest {
        SendEmailRequest {
            destination: Destination {
                to_addresses: Some(self.recipients.to_owned()),
                ..Default::default()
            },
            message: Message {
                body: Body {
                    html: Some(Content {
                        charset: Some(CHARSET.to_owned()),
                        data: self.html_body.to_string(),
                    }),
                    text: Some(Content {
                        charset: Some(CHARSET.to_owned()),
                        data: self.raw_text_body.to_string(),
                    }),
                },
                subject: Content {
                    charset: Some(CHARSET.to_owned()),
                    data: self.subject.to_string(),
                },
            },
            source: self.sender.to_owned(),
            ..Default::default()
        }
    }
}
