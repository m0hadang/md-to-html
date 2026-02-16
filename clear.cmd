@echo off
setlocal
REM --- Change these to match config in src/config.rs ---
set "OUTPUT_DIR=post"
set "SOURCE_DIR=src-post"
set "ROOT_LISTING=%OUTPUT_DIR%.md"
REM ------------------------------------------------

REM Remove generated output
rd /s /q "%OUTPUT_DIR%" 2>nul

REM Remove indexing md files (post.md, rust.md, etc. - one per dir under source)
del /q "%SOURCE_DIR%\%ROOT_LISTING%" 2>nul
for /d /r "%SOURCE_DIR%" %%D in (*) do (
  if exist "%%D\%%~nxD.md" del /q "%%D\%%~nxD.md"
)
