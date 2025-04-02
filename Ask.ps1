Write-Host "Whatever clipboard you had is going to be used as context"
Write-Host "I recommend copying terminal using ctrl+shift+uparrow followed by ctrl+c"
$x=Get-Clipboard -Raw
sd
$y = Get-Clipboard -Raw
$z = $x + "`n`n`n" + $y
$question = Read-Host "Enter your question"
Write-Output $z | ask $question