Copy-Item target/debug/XSNotify.exe ./windows-packaging
Set-Location windows-packaging
& "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\makeappx.exe" pack /d . /p ../out/XSNotify.msix /nv /o
& "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\signtool.exe" sign /a /v /fd SHA256 /f ../../common/mycert.pfx /p qwertyuiop ../out/XSNotify.msix
