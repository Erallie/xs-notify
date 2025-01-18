<section class="space-y-12">
    <h1 class="text-center text-4xl font-extrabold text-nowrap tracking-wide">
        About
    </h1>
    <div class="space-y-4 text-pretty">
        <h2 class="text-center text-2xl font-bold">Installation</h2>
        <p>
            XS Notify was forked from <a
                href="https://github.com/bluskript/xsoverlay-notifier"
                target="_blank"
                class="link">XSOverlay Notifier</a
            >
            and modified by
            <a href="https://github.com/Erallie" target="_blank" class="link"
                >Erika Gozar</a
            >, with the help of Zelaf and Jakob.
        </p>
        <p>
            This is a desktop application that runs alongside XSOverlay and
            sends all windows desktop notifications to display in VR. It uses
            the <a
                href="https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/notification-listener"
                target="_blank"
                class="link"
            >
                Windows Notification Listener API</a
            >
            to listen for notifications.
        </p>
    </div>

    <div
        role="alert"
        class="alert justify-items-start bg-base-300 shadow-xl text-left"
    >
        <div class="text-xl font-extrabold text-error">Important Note</div>
        <p>
            In order for XS Notify to relay notifications to XS Overlay, the
            notification toast <em>must</em> show up on your screen. If the toast
            does not appear on your screen (like for example, if the toast is blocked
            because you are in full screen), the notification will not be pushed
            to XSOverlay.
        </p>
    </div>

    <div class="space-y-4 text-pretty">
        <h2 class="text-center text-2xl font-bold">Installation</h2>
        <ol class="list-decimal ml-8">
            <li>
                Head over to the <a
                    href="https://github.com/Erallie/xs-notify/releases"
                    target="_blank"
                    class="link">Releases</a
                >
                page and find the latest release.
            </li>
            <li>
                Under <strong>Assets</strong>, download
                <code>xs-notify.exe</code>.
            </li>
            <li>
                Run <code>xs-notify.exe</code>.
            </li>
            <li>
                Start the notification bridge when you want your notifications
                pushed to XSOverlay.
            </li>
        </ol>
    </div>

    <div class="space-y-4 text-pretty">
        <h2 class="text-center text-2xl font-bold">Auto-launch XS Notify</h2>
        <p>
            XS Notify has a setting you can enable to have it launch
            automatically when you start your computer.
        </p>
        <p>
            If you would rather have XS Notify launch automatically when you
            launch VRChat, you can use <a
                href="https://github.com/vrcx-team/VRCX"
                target="_blank"
                class="link">VRCX</a
            > and add the shortcut to your Auto-launch folder by following these
            steps:
        </p>
        <ol class="list-decimal ml-8">
            <li>
                <strong>Right-click</strong> the executable, then click
                <strong>Show more options → Create shortcut</strong>.
            </li>
            <li>Open VRCX.</li>
            <li>
                Go to <strong
                    >Settings → Advanced → App Launcher → Auto-Launch Folder</strong
                >.
            </li>
            <li>Move the shortcut to the folder that opens.</li>
        </ol>
    </div>

    <div class="space-y-4 text-pretty">
        <h2 class="text-center text-2xl font-bold">Support</h2>
        <p>
            If you have any problems with this tool or want to request a
            feature, please create an <a
                href="https://github.com/Erallie/xs-notify/issues"
                target="_blank"
                class="link">issue</a
            >, and I will try to address it to the best of my ability!
        </p>
    </div>
</section>
