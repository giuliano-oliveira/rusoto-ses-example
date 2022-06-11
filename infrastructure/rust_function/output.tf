output "arn" {
  description = "The ARN of the Lambda Function"
  value       = aws_lambda_function.this.arn
  sensitive   = true
}

output "function_name" {
  description = "The name of the Lambda Function"
  value       = aws_lambda_function.this.function_name
  sensitive   = true
}
