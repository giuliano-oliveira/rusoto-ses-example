provider "aws" {
  region = var.region

  # Make it faster by skipping something
  skip_get_ec2_platforms      = true
  skip_metadata_api_check     = true
  skip_region_validation      = true
  skip_credentials_validation = true
  skip_requesting_account_id  = true
}

resource "aws_iam_role" "lambda_role" {
  name = "lambda_role-${local.environment}"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        },
        Action = "sts:AssumeRole"
      },
    ]
  })
}

resource "aws_iam_role_policy" "lambda_role_policy" {
  name = "lambda_role_policy-${local.environment}"
  role = aws_iam_role.lambda_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "ses:SendEmail",
        ]
        Effect   = "Allow"
        Resource = "*"
      },
    ]
  })
}

module "send_my_email_function" {
  source          = "./rust_function"
  function_path   = "../target/lambda/rusoto-ses-example/bootstrap.zip"
  function_name   = "${local.prefix}send-my-email"
  lambda_role_arn = aws_iam_role.lambda_role.arn
  description     = "Example rusoto SES sender"

  environment_variables = {
    SENDER = var.sender
  }

  tags = local.default_tags
}
