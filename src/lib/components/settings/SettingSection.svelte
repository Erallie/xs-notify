<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        title = "Section Title",
        description = "Section Description",
        children,
    }: {
        title: string;
        description: string;
        children?: Snippet<[]>;
    } = $props();
</script>

<!--
@component
A wrapper for different sections of the settings page.

# Properties:
- `title` — The title of the section.
- `description` — The description or short summary of the section.

# Usage:
```svelte
    <SettingSection
        title="Section Title"
        description="A short summary or description of the section">
        <div>Content inside section</div>
    </SettingSection>
```
-->
<section class="max-w-2xl mx-auto">
    <h2 class="divider sm:divider-start text-xl font-bold">{title}</h2>
    <p>{description}</p>

    <div class="space-y-8 mt-8">
        {@render children?.()}
    </div>
</section>
