@echo off
REM Remove generated output
rd /s /q post 2>nul

REM Remove indexing md files (post.md, rust.md, etc. - one per dir under src-post)
del /q src-post\post.md 2>nul
for /d /r "src-post" %%D in (*) do (
  if exist "%%D\%%~nxD.md" del /q "%%D\%%~nxD.md"
)
