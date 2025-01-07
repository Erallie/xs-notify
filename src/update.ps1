param(
    [string]$TempFilePath,
    [string]$ExePath
)

# Get the path of the currently running script
$currentScriptPath = $MyInvocation.MyCommand.Path

Start-Sleep -Seconds 1

# Check if the file exists
if (Test-Path $TempFilePath) {
    # Remove the file
    Copy-Item $TempFilePath $ExePath
    Remove-Item $TempFilePath -Force
    Start-Process -FilePath $ExePath
    # Remove the script file
    Remove-Item -Path $currentScriptPath -Force
    exit
} else {
    Write-Host "File does not exist."
}