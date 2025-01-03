cargo build --release

Write-Output "Release: "
$time1 = Measure-Command { .\target\release\zx.exe -lha ../../ }
$milliseconds1 = $time1.TotalMilliseconds
Write-Output "The command took $milliseconds1 ms"

Write-Output "Latest version: "
$time2 = Measure-Command { zx -la }
$milliseconds2 = $time2.TotalMilliseconds
Write-Output "The command took $milliseconds2 ms"
