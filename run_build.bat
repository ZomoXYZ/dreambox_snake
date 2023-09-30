@echo off
dbsdk-cli build .
cd ..\dreambox-windows-alpha
dreambox.exe -f ..\snake_dreambox\build\debug.iso
cd ..\snake_dreambox
