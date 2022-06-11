mod send_my_email;
use dotenv::dotenv;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rusoto_core::Region;
use rusoto_ses::SesClient;
use send_my_email::send_my_email;
use serde::{Deserialize, Serialize};
use serde_env::from_env;
use serde_json::{from_value, json, Value};
use ses_mailer::SesMailer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv();
    let handler = service_fn(handle);
    lambda_runtime::run(handler).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct LambdaSettings {
    sender: String,
}

#[derive(Serialize, Deserialize)]
struct LambdaInput {
    recipients: Vec<String>,
    message: String,
}

async fn handle(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let settings: LambdaSettings = from_env()?;
    let input: LambdaInput = from_value(event)?;

    let client = SesClient::new(Region::default());
    let mailer = SesMailer::new(client);
    send_my_email(
        &mailer,
        &settings.sender,
        &input.recipients.to_owned(),
        &input.message.to_string(),
    )
    .await
    .unwrap();

    Ok(json! ({
        "message": "Email sent!"
    }))
}
