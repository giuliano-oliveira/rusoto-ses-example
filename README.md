# An AWS Lambda SES Sender example in rust using rusoto
An example of using rusoto to send emails in AWS Lambda and deploying it using terraform.

## Pre-requirements
* cargo
* [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)
* terraform
* make

## Setup
* Setup your aws config [(docs)](https://docs.aws.amazon.com/cli/latest/reference/configure/);
* Copy the file `.env.default` to `.env` and edit the default environment variables;
* Make sure the email supplied in the `SENDER` environment variable is verified in SES [(docs)](https://docs.aws.amazon.com/ses/latest/dg/creating-identities.html#just-verify-email-proc).

## Unit testing
Execute:
```sh
make test
```

## Offline development
Start the emulator with:
```sh
cargo lambda watch
```
and then, in another terminal:
```sh
cargo lambda invoke --data-ascii '{"recipients": ["recipient@email.com"], "message": "offline test"}'
```

## Deploying
For development stage:
```sh
make lambda-deploy-development
```

For production stage:
```sh
make lambda-deploy-production
```

After that you will be prompted to set the `sender` variable, you can set it beforehand with the env `TF_VAR_sender`.
