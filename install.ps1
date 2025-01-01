#Requires -RunAsAdministrator
$Cert = "C:\Users\EDSGo\Documents\Forks\Certificates/certificate.cer"
$Installer = New-TemporaryFile
Invoke-WebRequest -Uri "https://github.com/Erallie/xs-notify/releases/download/latest/XSNotify.msix "-OutFile $Installer
Import-Certificate -FilePath $Cert -Cert Cert:\CurrentUser\My
Add-AppPackage -Path $Installer
Start-Process "shell:AppsFolder\$((Get-StartApps | Where-Object {$_.Name -eq 'XSNotify'}).'AppID')"
if ($AppID) {
    Start-Process "shell:AppsFolder\$AppID"
}
else {
    Write-Host "Application 'XSNotify' not found in Start Menu."
}
