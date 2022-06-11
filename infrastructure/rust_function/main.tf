resource "aws_lambda_function" "this" {
  role             = var.lambda_role_arn
  publish          = true
  function_name    = var.function_name
  runtime          = "provided.al2"
  filename         = var.function_path
  handler          = "doesnt.matter"
  timeout          = var.timeout
  memory_size      = var.memory_size
  source_code_hash = filebase64sha256(var.function_path)
  tags             = var.tags
  description      = var.description


  dynamic "environment" {
    for_each = length(keys(var.environment_variables)) == 0 ? [] : [true]
    content {
      variables = var.environment_variables
    }
  }
}
