<script lang="ts">
    import type { ChangeEventHandler, HTMLAttributes } from "svelte/elements";

    let {
        checked = $bindable(),
        divClass,
        class: className,
        onchange,
        label,
        description,
    }: {
        /**
         * *Optional*, *Bindable*
         *
         * If the switch is on or not.
         */
        checked?: boolean;
        /**
         * *Optional*
         *
         * Additional classes to add to the wrapper element.
         */
        divClass?: HTMLAttributes<HTMLInputElement>["class"];
        /**
         * *Optional*
         *
         * Additional classes to set on the input element.
         */
        class?: HTMLAttributes<HTMLDivElement>["class"];
        /**
         * *Optional*
         *
         * Callback function for when the switch turns on or off.
         */
        onchange?: ChangeEventHandler<HTMLInputElement>;
        /**
         * The label that appears next to the switch.
         */
        label: string;
        /**
         * *Optional*
         *
         * The description that appears under the switch.
         */
        description?: string;
    } = $props();
</script>

<!--
@component
A switch component to toggle a boolean value.

# Attributes:
- `checked` — If the switch is checked or not. *Optional*, *Bindable*
- `divClass` — Add additional CSS classes to the wrapper Element. *Optional*
- `class` — Add additional CSS classes to the input element. *Optional*
- `onchange` — A callback function that runs every time the switch is changed. *Optional*
- `label` — The label appearing next to the switch.
- `description` - A description appearing below the switch. *Optional*

# Usage:
```svelte
    <Switch
        bind:checked={runOnBoot}
        class="toggle-primary"
        label="Run on boot"
        description="Runs XSNotify on boot." />
```
-->

<div class={className}>
    <label class="label cursor-pointer">
        <span class="label-text text-lg font-semibold">{label}</span>
        <input
            {onchange}
            type="checkbox"
            class="toggle {className}"
            bind:checked />
    </label>
    {#if description}
        <div class="label-text text-pretty">{description}</div>
    {/if}
</div>
