#Requires -RunAsAdministrator
$Installer = New-TemporaryFile
Invoke-WebRequest -Uri https://github.com/Erallie/xs-notify/releases/download/latest/XSNotify.msix -OutFile $Installer
# Import-Certificate -FilePath $Cert -Cert Cert:\CurrentUser\My
Add-AppPackage -AllowUnsigned -Path $Installer
Start-Process "shell:AppsFolder\$((Get-StartApps | Where-Object {$_.Name -eq 'XSNotify'}).'AppID')"
