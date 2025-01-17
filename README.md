# XS Notify
Forked from [XSOverlay Notifier](https://github.com/bluskript/xsoverlay-notifier).

This is a desktop application that runs alongside XSOverlay and pushes your windows desktop notifications to display in VR. It uses the [Windows Notification Listener API](https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/notification-listener) to listen for notifications.

There is an option in the settings to specify a **whitelist/blacklist** to include or exclude certain apps from being pushed to XSOverlay.

> ### IMPORTANT NOTE
> In order for XS Notify to relay notifications to XS Overlay, the notification toast *must* show up on your screen. If the toast does not appear on your screen (like for example, if the toast is blocked because you are in full screen), the notification will not be pushed to XSOverlay.
# Installation
1. Head over to the [Releases](https://github.com/Erallie/xs-notify/releases) page and find the latest release.
2. Under **Assets**, download `xs-notify.exe`
3. Run `xs-notify.exe`.
4. Start the notification bridge when you want your notifications pushed to XSOverlay.
# Auto-launch XS Notify
XS Notify has a setting you can enable to have it launch automatically when you start your computer.

If you would rather have XS Notify launch automatically when you launch VRChat, you can use [VRCX](https://github.com/vrcx-team/VRCX) and add the shortcut to your Auto-Launch folder by following these steps:
1. **Right-click** the executable, then click **Show more options → Create shortcut**.
2. Open VRCX.
3. Go to **Settings → Advanced → App Launcher → Auto-Launch Folder**.
4. Move the shortcut to the folder that opens.
# Support
If you have any problems with this tool or want to request a feature, please create an [issue](https://github.com/Erallie/xs-notify/issues), and I will try to address it to the best of my ability!