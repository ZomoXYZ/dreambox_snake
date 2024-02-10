@echo off
@REM Set the path to the folder containing dreambox.exe
set DREAMBOX_PATH=..\dreambox

set HERE_PATH=%cd%
dbsdk-cli build .
cd %DREAMBOX_PATH%
dreambox.exe -f %HERE_PATH%\build\debug.iso
cd %HERE_PATH%
