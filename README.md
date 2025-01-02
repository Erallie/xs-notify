# XS Notify
Forked from [XSOverlay Notifier](https://github.com/bluskript/xsoverlay-notifier).

This is a desktop application that runs alongside XSOverlay and sends all windows desktop notifications to display in VR. It uses the [Windows Notification Listener API](https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/notification-listener) to listen for notifications.

> ### IMPORTANT NOTE
> In order for XS Notify to relay notifications to XS Overlay, the notification toast *must* show up on your screen. If the toast does not appear on your screen (like for example, if the toast is blocked because you are in full screen), the notification will not be pushed to XSOverlay.
# Installation
1. Head over to the [Releases](https://github.com/Erallie/xs-notify/releases) page and find the latest release.
2. Under **Assets**, download `xs_notify.exe`
3. Run `xs_notify.exe` when you want your notifications pushed to XSOverlay.
# Auto-launch XS Notify
Here are some step-by-step instructions on how to set up XS Notify to launch automatically.
## On System Startup
To have XS Notify launch automatically when you start your computer, follow these steps:
1. **Right-click** the executable, then click **Show more options → Create shortcut**.
2. Open File Explorer.
3. Type `shell:startup` in the address bar and press **Enter**.
4. Move the shortcut to the folder that opens.
## When launching VRChat
To have XS Notify launch automatically when you launch VRChat, you can use [VRCX](https://github.com/vrcx-team/VRCX) and add the shortcut to your Auto-Launch folder by following these steps:
1. **Right-click** the executable, then click **Show more options → Create shortcut**.
2. Open VRCX.
3. Go to **Settings → Advanced → App Launcher → Auto-Launch Folder**.
4. Move the shortcut to the folder that opens.
# Configuration
Settings for XS Notify are stored in `%APPDATA%\Gozar Productions LLC\XS Notify\config\config.toml`. Here is some brief documentation on each option in the config:
```toml
# Port that xsoverlay is listening on
port = 42069
# The hostname that xsoverlay is listening on
host = "localhost"
# The notification strategy - either "listener" or "polling"
notification_strategy = "polling"
# The rate at which the polling strategy refreshes notifications
polling_rate = 250
# Set this to "true" if you want the the notification display time to change depending on the amount of text being displayed. Otherwise, set it to "false".
dynamic_timeout = true
# The duration the notification shows up on screen when dynamic_timeout is disabled.
default_timeout = 5
# Your reading speed in words per minute (WPM). When dynamic_timeout is enabled, the timeout of each notification will be calculated based off of this.
reading_speed = 238
# When dynamic_timeout is enabled, notifications will never display for less than this many seconds.
min_timeout = 2
# When dynamic_timeout is enabled, notifications will never display for more than this many seconds.
max_timeout = 120
# If there are apps you don't want XS Notify to push notifications for, add their names in quotation marks ( "" ) to the brackets below, separated by commas ( , ).
# It should look something like this: ["Example App 1", "Example App 2"]
skipped_apps = []
```
# Support
If you have any problems with this tool or want to request a feature, please create an [issue](https://github.com/Erallie/xs-notify/issues), and I will try to address it to the best of my ability!