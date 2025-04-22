# XS Notify

[![Latest Release](https://img.shields.io/github/release-date/Erallie/xs-notify?display_date=published_at&style=for-the-badge&label=Latest%20Release)](https://github.com/Erallie/xs-notify/releases/latest)
![Latest Downloads](https://img.shields.io/github/downloads/Erallie/xs-notify/latest/total?sort=semver&style=for-the-badge&label=Latest%20Downloads)
![All-time Downloads](https://img.shields.io/github/downloads/Erallie/xs-notify/total?style=for-the-badge&label=Downloads)
<br>
[![Our Discord](https://img.shields.io/discord/1102582171207741480?style=for-the-badge&logo=discord&logoColor=ffffff&label=Our%20Discord&color=5865F2)](https://discord.gozarproductions.com)
[![Our Other Projects](https://img.shields.io/badge/Our%20Other%20Projects-%E2%9D%A4-563294?style=for-the-badge&logo=data%3Aimage%2Fwebp%3Bbase64%2CUklGRu4DAABXRUJQVlA4WAoAAAAQAAAAHwAAHwAAQUxQSGABAAABgFtbm5volyZTA%2BtibzK2H0w5sDkmhe3GmxrwxGg0839r%2FvkkOogIBW7bKB0c4%2BARYihzIqfd6dfO%2B%2B3XtHsq4jJhlIvcDRcgNB%2FeieQETorBHgghRtUYqwDs%2B4U4IpcvUB%2BVUPSK54uEnTwsUJoar2DeMpzLxQpeG5DH8lxyyfLivVYAwPBbkWdOBg3qFlqiLy679iHy9UDKMZRXmYxpCcusayTHG01K%2FEtatYWuj7oI9hL4BxsxVwhoP2mlAJJ%2BuuAflc6%2BEUCQTCX9EV87xBR2H75NxLZSpWiwzqdIm7ZO7uB3oEgZKbD9Nt3EmHweEPH1t1GNsZUbKeisiwjyTm5fA3SO1yCrADZXrV2PZQJPL1tjN4%2BxUL9ie1mJobzOnDwSx6ILiF%2FW%2BTUR4tcHx0UaV75JXC1a4g6Ky5dLcTSuy9q4HhTieF64Hy1A3GHB8gLLK2e92feuqnbfPK8IVlA4IGgCAACwDQCdASogACAAPk0cjEQioaEb%2BqwAKATEtgBOl7v9V3sHcA2wG4A3gD0APLP9jX9n%2F2jmqv5AZRh7J%2BN2fOx22iE%2F4TUsecFmY%2BSf1r%2BAP%2BTfzT%2FXdIB7KX7MtdIGr1A8H0jmrrfZvqButwOaYcLWYNRq5QgAAP7%2F%2FmIMpiVNn67QXpM1rrDmRS8Nr%2F6dhD%2Bq5e%2BM%2BAtUP1%2FxOj85Ol5y3ebjz%2BpHoOf%2FWW8a%2F2ojUaKVDkVqof%2Bv4f0f6ud8i58wusz%2Fyrj%2F%2BwnM3q0769dvK%2F%2BQe04xL49tkb9t6ylCqqezZtZGuGLJ%2F5iUrPqdYc%2F8VbYZfP%2FOpZP%2F4X4q%2BqS4gPOxzdINOe5PGv%2F0TS%2FJRf4LlFrFkrWtxlS8n40grV%2BKUu%2FiwzdQzImvwH81FxL1bZyTSsrYwMku1Pk9StTtWNjSR8ZWEYBH9eTn%2FvBERii5XaWOPJ%2FFVXtVQGbv%2BFRW5jbo9tfFDu%2BDHHf8LbgUd%2F8W8Id1AehBtRNsLQWbADmvF1QJU8x5tw%2FtTUwIoSaa%2F2jkcvyVHkAsb2qoIh1KF1pPdae%2BZaqjydy6nUa9agjrDk1G4pMhEUhH%2BV%2FIUe49MjhR%2FuxyFmwQ8dDogMyQ%2BdcSBa56Lwt1wyJ%2F22%2F5O98r6q6wiM63HyaYONd36W7br%2F0%2F6y2DZ3irAddj%2FRxntvr%2FbbChSYXAfEbO%2FD0G%2FFbMFqTHypodt9T6dAx%2BUjJYfHzFf%2FM3Ec%2FAtwbjc2gka6urN1MlSLb2VTS9Q5r8fkDzxZz6vu1OYUPUB1UFMIhYGvMATbxxoTmVhvpovzAc%2F8nbOjw3wAAA)](https://github.com/Erallie)
[![Donate](https://img.shields.io/badge/Donate-%24-563294?style=for-the-badge&logo=paypal&color=rgb(0%2C%2048%2C%20135))](https://www.paypal.com/donate/?hosted_button_id=PHHGM83BQZ8MA)

---

Forked from [XSOverlay Notifier](https://github.com/bluskript/xsoverlay-notifier).

This is a desktop application that runs alongside [XSOverlay](https://store.steampowered.com/app/1173510/XSOverlay/) and pushes your windows desktop notifications to display in VR. It uses the [Windows Notification Listener API](https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/notification-listener) to listen for notifications.

> ### IMPORTANT NOTE
> In order for XS Notify to relay notifications to XS Overlay, the notification toast *must* show up on your screen. If the toast does not appear on your screen (like for example, if the toast is blocked because you are in full screen), the notification will not be pushed to XSOverlay.
# Key Features
- An option to specify a **whitelist/blacklist** to include or exclude certain apps from being pushed to XSOverlay.
- An option to have the **display time** of notifications be **dynamically set** based on the amount of text in a notification and your reading speed.
- A nice **interface** for the settings with a tab for the console.
- Options to **launch on system startup** and **minimize to the system tray**.
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