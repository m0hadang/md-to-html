@echo off
cargo build --release
if exist target\release\md-to-html.exe (
    copy /Y target\release\md-to-html.exe .\md-to-html.exe >nul
    echo Copied to .\md-to-html.exe
)
