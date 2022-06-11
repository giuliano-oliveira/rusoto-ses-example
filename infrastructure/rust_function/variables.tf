variable "function_path" {
  type        = string
  description = "Function zip path"
}

variable "function_name" {
  type        = string
  description = "Function name"
}

variable "lambda_role_arn" {
  type        = string
  description = "Lambda role arn"
}

variable "timeout" {
  type        = number
  description = "Lambda timeout"
  default     = 30
}
variable "memory_size" {
  type        = number
  description = "Lambda memory size (must be multiple of 128)"
  default     = 128
}

variable "environment_variables" {
  type        = map(string)
  description = "Lambda environment variables"
  default     = {}
}

variable "tags" {
  type        = map(string)
  description = "Lambda tags"
  default     = {}
}

variable "description" {
  type        = string
  description = "Lambda description"
  default     = null
}
