$jsonUrl = "https://raw.githubusercontent.com/arkeasz/zix/refs/heads/main/zix.json"

try {
    $json = Invoke-RestMethod -Uri $jsonUrl -Method GET
    $latestVersion = $json."dist-version".latest
    $latestUrl = $json.versions.$latestVersion.url
    Write-Output $json
    Write-Host "Latest available version: $latestVersion"

    $TargetDir = "C:\Program Files\Zix"
    New-Item -ItemType Directory -Path $TargetDir -Force
    $DownloadPath = Join-Path -Path $TargetDir -ChildPath "zx.exe"

    Invoke-WebRequest -Uri $latestUrl -OutFile $DownloadPath

    [Environment]::SetEnvironmentVariable("PATH", "$env:PATH;$TargetDir", [EnvironmentVariableTarget]::Machine)
    Write-Output "zix successfully installed and added to PATH."

} catch {
    Write-Host "Error fetching registry information: $_"
}
