output "lambda_function_arn" {
  description = "The ARN of the Lambda Function"
  value       = module.send_my_email_function.arn
  sensitive   = true
}

output "lambda_function_dashboard_url" {
  description = "Lambda Function Dashboard"
  value       = "https://${var.region}.console.aws.amazon.com/lambda/home?region=${var.region}#/functions/${module.send_my_email_function.function_name}?tab=testing"
  sensitive   = true
}
