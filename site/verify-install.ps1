$ErrorActionPreference = "Stop"
Invoke-Expression ((Invoke-WebRequest -UseBasicParsing -Uri "https://raw.githubusercontent.com/TheAstrayDev/astray-verify/main/install.ps1").Content)
