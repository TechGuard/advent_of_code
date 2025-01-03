@echo off
if "%2" == "--make" (
    echo Unsupported command: --make
    goto :eof
)

FOR /F "tokens=*" %%F IN ('"%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -latest -prerelease -products * -requires Microsoft.Component.MSBuild -find MSBuild\**\Bin\MSBuild.exe') DO SET msbuild="%%F"
for /D %%x in (%1*) do if not defined f set "project=%%x"
%msbuild% %~dp0AdventOfCode2017.sln "-t:%project%" 1>nul
"%~dp0x64\Debug\%project%.exe" %2