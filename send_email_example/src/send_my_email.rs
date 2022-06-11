use askama::Template;
use ses_mailer::Email;
use traits::AsyncSender;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "email_template.html")]
struct EmailTemplate {
    message: String,
}

#[derive(PartialEq, Debug)]
pub enum SendMyEmailError<E> {
    TemplateError(String),
    SendError(E),
}

pub async fn send_my_email<E>(
    async_sender: &impl AsyncSender<Email, Error = E>,
    sender: &String,
    recipients: &Vec<String>,
    message: &String,
) -> Result<(), SendMyEmailError<E>> {
    let email_template = EmailTemplate {
        message: message.to_string(),
    };
    let html_body = email_template
        .render()
        .map_err(|e| SendMyEmailError::TemplateError(e.to_string()))?;
    async_sender
        .send(&Email {
            sender: sender.to_owned(),
            recipients: recipients.to_owned(),
            html_body: html_body.to_string(),
            raw_text_body: "".to_string(),
            subject: "Hello from rust 🦀".to_string(),
        })
        .await
        .map_err(|e| SendMyEmailError::SendError(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::{mock, predicate};

    #[derive(Debug, PartialEq)]
    enum MyError {}

    mock! {
        EmailAsyncSender{}

        #[async_trait]
        impl AsyncSender<Email> for EmailAsyncSender{
            type Error = MyError;
            async fn send(&self, input: &Email) -> Result<(), MyError>;
        }

    }

    #[tokio::test]
    async fn test_send_my_email() {
        let mut mocked_sender = MockEmailAsyncSender::new();

        let expected_email = Email {
            sender: "sender@email.com".to_string(),
            recipients: vec!["recipient@email.com".to_string()],
            html_body: include_str!("../templates_test/expected_html1.html").trim_end().to_string(),
            raw_text_body: "".to_string(),
            subject: "Hello from rust 🦀".to_string(),
        };

        mocked_sender
            .expect_send()
            .with(predicate::eq(expected_email))
            .times(1)
            .returning(|_| Ok(()));
        let res = send_my_email(
            &mocked_sender,
            &"sender@email.com".to_owned(),
            &vec!["recipient@email.com".to_owned()],
            &"hello world".to_string(),
        )
        .await;
        assert!(res == Ok(()));
    }
}
