#Requires -RunAsAdministrator
# $Cert = "C:\Users\EDSGo\Documents\Forks\Certificates\certificate.cer"
$Installer = New-TemporaryFile
Invoke-WebRequest -Uri "https://github.com/Erallie/xs-notify/releases/download/v0.0.0.2-alpha/XSNotify.msix" -OutFile $Installer
# Import-Certificate -FilePath $Cert -Cert Cert:\CurrentUser\Root
Add-AppPackage -AllowUnsigned -Path $Installer
Start-Process "shell:AppsFolder\$((Get-StartApps | Where-Object {$_.Name -eq 'XS Notify'}).'AppID')"
if ($AppID) {
    Start-Process "shell:AppsFolder\$AppID"
}
else {
    Write-Host "Application 'XS Notify' not found in Start Menu."
}
