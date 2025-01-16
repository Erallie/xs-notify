<script lang="ts">
    import AboutSection from "./AboutSection.svelte";
</script>

<section class="container space-y-12">
    <h1 class="text-center text-4xl font-extrabold text-nowrap tracking-wide">
        About
    </h1>
    <AboutSection title="General"
        ><p>
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
        <div class="collapse collapse-open border-base-300 bg-base-300">
            <div class="collapse-title text-xl font-extrabold text-warning">
                Important Note
            </div>
            <div class="collapse-content">
                <p>
                    In order for XS Notify to relay notifications to XS Overlay,
                    the notification toast <em>must</em> show up on your screen.
                    If the toast does not appear on your screen (like for example,
                    if the toast is blocked because you are in full screen), the
                    notification will not be pushed to XSOverlay.
                </p>
            </div>
        </div></AboutSection
    >
    <AboutSection title="Auto-launch XS Notify"
        ><p>
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
        </ol></AboutSection
    >
    <AboutSection title="Support">
        <p>
            If you have any problems with this tool or want to request a
            feature, please create an <a
                href="https://github.com/Erallie/xs-notify/issues"
                target="_blank"
                class="link">issue</a
            >, and I will try to address it to the best of my ability!
        </p>
    </AboutSection>
</section>
