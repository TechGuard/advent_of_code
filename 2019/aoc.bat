@echo off
if "%2" == "--make" (
    python make.py %*
) else (
    cargo run -r -- %*
)