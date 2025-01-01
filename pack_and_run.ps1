cargo build
./pack.ps1
Set-Location ..
Remove-AppPackage (Get-AppPackage -name 'GozarProductions.XSNotify').'PackageFullName'
Add-AppPackage -AllowUnsigned -Path "./out/XSNotify.msix"
Start-Process "shell:AppsFolder\$((Get-StartApps | Where-Object {$_.Name -eq 'XS Notify'}).'AppID')"