cargo build
./pack.ps1
Set-Location ..
Remove-AppPackage (Get-AppPackage -name 'GozarProductions.XSNotify').'PackageFullName'
Add-AppPackage -Path ./out/XSNotify.msix
Start-Process "shell:AppsFolder\$((Get-StartApps | Where-Object {$_.Name -eq 'XSNotify'}).'AppID')"