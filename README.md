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
* Make sure your current IAM user has the right permissions, example: `lambda:*`, `s3:*` and `iam:*`, 
to make a more fine-grained permission, you must check for the permissions to deploy a lambda,
creating an IAM Role, and read/writing to an S3 bucket.
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
### **Compiling and initializing terraform**
First compile the lambda code:
```sh
make lambda-build
```

set the environment variables:
* `BUCKET_NAME`: A pre-created s3 bucket that will store terraform state;
* `AWS_PROFILE`: your profile from `aws config`;
* `AWS_REGION`: default `us-east-1`.

Then, initialize terraform
```sh
make tf-init
```
### **Deploying to a stage**
* Development: `make lambda-deploy-development`
* Production: `make lambda-deploy-production`

After that you will be prompted to set the `sender` variable, you can set it beforehand with the env `TF_VAR_sender`.
