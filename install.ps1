# 管理者権限チェック
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {   
    Write-Warning "管理者権限が必要です。管理者として実行してください。"
    break
}

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

# インストール先ディレクトリの設定
$installDir = "$env:LocalAppData\Programs\gather_files"
$output = "$installDir\gather_files.exe"

# インストールディレクトリの作成
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

Write-Output "Downloading $download..."
Invoke-WebRequest $download -Out $output

Write-Output "Installing to $output"

# 環境変数PATHの更新
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if (-not $currentPath.Contains($installDir)) {
    Write-Output "Adding installation directory to system PATH..."
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$installDir",
        "Machine"
    )
    $env:Path = "$env:Path;$installDir"
    Write-Output "PATH has been updated successfully."
}

Write-Output "Installation completed!"
Write-Output "You can now run 'gather_files' from your terminal."