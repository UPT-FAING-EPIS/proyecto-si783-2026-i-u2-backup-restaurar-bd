output "server_public_ip" {
  description = "Dirección IP pública del servidor"
  value       = aws_instance.app_server.public_ip
}

output "server_public_dns" {
  description = "DNS público del servidor"
  value       = aws_instance.app_server.public_dns
}
