locals {
  environment = terraform.workspace == "default" ? "production" : terraform.workspace
  prefix      = local.environment == "production" ? "" : "${local.environment}-"
  default_tags = {
    ENVIRONMENT = local.environment
  }
}
