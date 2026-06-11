variable "aws_region" {
  description = "Región de AWS para desplegar la infraestructura"
  type        = string
  default     = "us-east-1"
}

variable "project_name" {
  description = "Nombre del proyecto para nombrar recursos"
  type        = string
  default     = "general-project"
}

variable "ami_id" {
  description = "ID de la AMI (ej. Ubuntu)"
  type        = string
  default     = "ami-0c55b159cbfafe1f0" 
}

variable "instance_type" {
  description = "Tipo de instancia EC2"
  type        = string
  default     = "t2.micro"
}
