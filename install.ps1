# バイパス設定を一時的に適用
Set-ExecutionPolicy Bypass -Scope Process -Force

$ErrorActionPreference = 'Stop'

$repo = "herring101/gather_files"
$file = "gather_files-windows-amd64.exe"
$releases = "https://api.github.com/repos/$repo/releases"

Write-Output "Fetching latest release..."
$tag = (Invoke-WebRequest $releases | ConvertFrom-Json)[0].tag_name

$download = "https://github.com/$repo/releases/download/$tag/$file"
$name = $file.Split(".")[-2]
$output = "$env:USERPROFILE\.cargo\bin\$name.exe"

# .cargo\binディレクトリが存在しない場合は作成
$binPath = "$env:USERPROFILE\.cargo\bin"
if (-not (Test-Path $binPath)) {
    New-Item -ItemType Directory -Path $binPath -Force | Out-Null
}

Write-Output "Downloading $download..."
Invoke-WebRequest $download -Out $output

Write-Output "Installing to $output"
$(Get-Item $output).Attributes += 'ReadOnly'

# PATHに.cargo\binが含まれているか確認
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not $currentPath.Contains($binPath)) {
    Write-Output "Adding .cargo\bin to your PATH..."
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$binPath",
        "User"
    )
    Write-Output "PATH has been updated. You may need to restart your terminal."
}

Write-Output "Installation completed!"
Write-Output "You can now run 'gather_files' from your terminal."