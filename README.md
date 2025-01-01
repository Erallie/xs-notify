# XS Notify
Forked from [XSOverlay Notifier](https://github.com/bluskript/xsoverlay-notifier).

This is a desktop application that runs alongside XSOverlay and sends all windows notifications to display in VR. It uses the [Windows Notification Listener API](https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/notification-listener) to listen for notifications.
## Installation
1. Head over to the [Releases](https://github.com/Erallie/xs-notify/releases) page and find the latest release.
2. Under **Assets**, download `xs_notify.exe`
3. Run `xs_notify.exe` when you want your notifications pushed to XSOverlay.
## Auto-launch XS Notify
### On System Startup
1. **Right-click** the executable, then click **Show more options → Create shortcut**.
2. Open File Explorer.
3. Type `shell:startup` in the address bar and press **Enter**.
4. Move the shortcut to the folder that opens.
### When launching VRChat
This method assumes you have [VRCX](https://github.com/vrcx-team/VRCX) installed.
1. **Right-click** the executable, then click **Show more options → Create shortcut**.
2. Open VRCX.
3. Go to **Settings → Advanced → App Launcher → Auto-Launch Folder**.
4. Move the shortcut to the folder that opens.
## Configuration
Settings for the notifier are stored in `%APPDATA%\gozar_productions\xs_notify\config\config.toml`. Here is some brief documentation on each option in the config:
```toml
# Port that xsoverlay is listening on
port = 42069
# The hostname that xsoverlay is listening on
host = "localhost"
# The notification strategy - either "listener" or "polling"
notification_strategy = "polling"
# The rate at which the polling strategy refreshes notifications
polling_rate = 250
# The duration the notification shows up on screen
timeout = 2
# Your reading speed in words per minute (WPM). When dynamic_timeout is enabled, the timeout of each notification will be calculated based off of this.
reading_speed = 238
# When dynamic_timeout is enabled, notifications will never display for less than this many seconds.
min_timeout = 2
# When dynamic_timeout is enabled, notifications will never display for more than this many seconds.
max_timeout = 120
```
