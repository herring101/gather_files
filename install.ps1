$ErrorActionPreference = 'Stop'

$repo = "herring101/gather_files"
$file = "gather-windows-amd64.exe"
$releases = "https://api.github.com/repos/$repo/releases"

Write-Output "Fetching latest release..."
$tag = (Invoke-WebRequest $releases | ConvertFrom-Json)[0].tag_name
$download = "https://github.com/$repo/releases/download/$tag/$file"

$installDir = "$env:LocalAppData\Programs\gather"
$output = "$installDir\gather.exe"

if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

Write-Output "Downloading $download..."
Invoke-WebRequest $download -Out $output

Write-Output "Installing to $output"

$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if (-not $currentPath.Contains($installDir)) {
    Write-Output "Adding installation directory to system PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "Machine")
    $env:Path = "$env:Path;$installDir"
}

Write-Output "Installation completed!  Run 'gather' from any terminal."
