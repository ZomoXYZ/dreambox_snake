@echo off
dbsdk-cli build .
cd ..\dreambox
dreambox.exe -f ..\snake_dreambox\build\debug.iso
cd ..\snake_dreambox
