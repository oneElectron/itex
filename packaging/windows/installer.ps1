if (Test-Path $env:LOCALAPPDATA/itex) {
    Remove-Item $env:LOCALAPPDATA/itex -Recurse -Force
}
New-Item $env:LOCALAPPDATA/itex -ItemType Directory

$RELEASE_NAME = (Invoke-WebRequest 'https://api.github.com/repos/oneelectron/itex/releases/latest' -UseBasicParsing | ConvertFrom-Json).tag_name.ToString()
$RELEASE_URL = "https://github.com/oneelectron/itex/archive/refs/tags/" + $RELEASE_NAME + ".zip"
$RELEASE_VERSION = $RELEASE_NAME.Replace("v", "")
Invoke-WebRequest -URI $RELEASE_URL -OutFile $env:LOCALAPPDATA/itex/itex.zip
Expand-Archive $env:LOCALAPPDATA/itex/itex.zip -DestinationPath $env:LOCALAPPDATA/itex/

Move-Item $env:LOCALAPPDATA/itex/itex-$RELEASE_VERSION/* $env:LOCALAPPDATA/itex
Remove-Item $env:LOCALAPPDATA/itex/itex-$RELEASE_VERSION -Recurse -Force
Remove-Item $env:LOCALAPPDATA/itex/itex.zip
New-Item -ItemType Directory $env:LOCALAPPDATA/itex/bin

$CUR_PATH = Get-ItemPropertyValue -Path "Registry::HKEY_CURRENT_USER\Environment" -Name "Path"
if (!$CUR_PATH.contains("itex")) {
    $env:Path += ";" + $env:LOCALAPPDATA + "\itex\bin;"
    Set-ItemProperty -Path "Registry::HKEY_CURRENT_USER\Environment" -Name Path ($CUR_PATH + ";" + $env:LOCALAPPDATA + "\itex\bin;")
}

Invoke-WebRequest ("https://github.com/oneelectron/itex/releases/download/" + $RELEASE_NAME + "/itex.exe") -OutFile $env:LOCALAPPDATA/itex/bin/itex.exe