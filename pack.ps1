Copy-Item target/debug/xs_notify.exe ./windows-packaging
Set-Location windows-packaging
& "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\makeappx.exe" pack /d . /p "../out/XSNotify.msix" /nv /o
# & "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\signtool.exe" sign /v /fd SHA256 /sha1 "031c5b631f8c5f1cf491ec25fd79193b5ea12e8c" ../out/XSNotify.msix