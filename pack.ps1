Copy-Item target/debug/xs_notify.exe ./windows-packaging
Set-Location windows-packaging
& "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\makeappx.exe" pack /d . /p "../out/XSNotify.msix" /nv /o
# & "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\signtool.exe" sign /v /fd SHA256 /sha1 "33a6288ccf42c31c34519c0ee38dc3055ee2882d" ../out/XSNotify.msix