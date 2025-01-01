Copy-Item "target/debug/XSNotify.exe" ./windows-packaging
Set-Location windows-packaging
& "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\makeappx.exe" pack /d . /p "../out/XSNotify.msix" /nv /o
& "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\signtool.exe" sign /v /fd SHA256 /sha1 "8e8de72715e8766b3053937b5827a2b534f25dfb" "../out/XSNotify.msix"