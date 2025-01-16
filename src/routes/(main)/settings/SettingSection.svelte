<script lang="ts">
    import { type SettingSection } from "$lib/types/types";

    let { title, description, children }: SettingSection = $props();
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
    </div>
</div>
