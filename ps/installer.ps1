$jsonUrl = "https://raw.githubusercontent.com/zix-rs/zix/refs/heads/main/zix.json"

try {
    $json = Invoke-RestMethod -Uri $jsonUrl -Method GET
    $latestVersion = $json.dist.latest
    $latestUrl = $json.versions.$latestVersion.win
    Write-Output $json
    Write-Host "Latest available version: $latestVersion"

    $TargetDir = "C:\Program Files\Zix"
    $DownloadPath = Join-Path -Path $TargetDir -ChildPath "zx.exe"

    $currentPath = [Environment]::GetEnvironmentVariable("PATH", [EnvironmentVariableTarget]::Machine)
    if ($currentPath -split ";" -contains $TargetDir) {
        Write-Host "Zix is already in PATH. Updating executable..."
    } else {
        Write-Host "Zix is not in PATH. Adding it now..."
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$TargetDir", [EnvironmentVariableTarget]::Machine)
        Write-Host "Zix added to PATH."
    }

    if (-not (Test-Path -Path $TargetDir)) {
        New-Item -ItemType Directory -Path $TargetDir -Force
    }

    Invoke-WebRequest -Uri $latestUrl -OutFile $DownloadPath
    Write-Output "Zix successfully installed/updated."

} catch {
    Write-Host "Error fetching registry information: $_"
}
