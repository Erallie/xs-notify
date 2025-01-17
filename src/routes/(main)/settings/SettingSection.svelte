<script lang="ts">
    import { type SettingSection } from "$lib/types/types";

    let { title, description, warning, children }: SettingSection = $props();
</script>

<div class="my-12">
    <h2 class="divider text-xl font-bold">{title}</h2>
    {#if description && description != ""}
        <p>
            {description}
        </p>
    {/if}
    <div class="py-4 space-y-8">
        {@render children()}
        {#if warning}
            <p class="text-warning">
                <span class="text-m">*</span><span class="text-xs"
                    >Changing these settings requires restarting the
                    notification bridge to take effect!</span
                >
            </p>
        {/if}
    </div>
</div>
