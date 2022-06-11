variable "region" {
  description = "AWS Region"
  default     = "us-east-1"
}

variable "sender" {
  description = "Email of the sender (Must be allowed in SES)"
  type        = string
}
