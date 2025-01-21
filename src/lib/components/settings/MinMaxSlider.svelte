<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";
    import NumberInputSmall from "./NumberInputSmall.svelte";
    import type { ChangeEventHandler, HTMLAttributes } from "svelte/elements";

    let {
        label,
        description,
        class: className,
        min,
        max,
        values = $bindable(),
        onchange,
    }: {
        /**
         * The label appearing above the slider.
         */
        label: string;
        /**
         * *Optional*
         *
         *  Sets the description label under the input
         */
        description?: string;
        /**
         * *Optional*
         *
         * Applies additional class names to the wrapper element.
         */
        class?: HTMLAttributes<HTMLDivElement>["class"];
        /**
         * The sliders minimum value.
         */
        min: number;
        /**
         * The sliders maximum value.
         */
        max: number;
        /**
         * *Bindable*
         *
         * The values of the slider
         */
        values: number[];
        /**
         * *Optional*
         *
         * Callback that runs every time the value changes
         */
        onchange?: ChangeEventHandler<HTMLElement>;
    } = $props();
</script>

<!--
@component
A slider with two handles to select a minimum and maximum value.

# Properties:
- `label` — Label appearing above the slider.
- `description` — Description appearing below the input. *Optional*
- `min` — The sliders minimum value.
- `max` — The sliders maximum value.
- `values` — The values of the input. Should be an array of two numbers. *Bindable*
- `class` — Extra classes to add to the wrapper element. *Optional*
- `onchange` — Callback for when the value changes. *Optional*

# Usage:
  ```svelte
  <MinMaxSlider
    label="Slider Label"
    description="Description of slider"
    min={10}
    max={90}
    bind:values={slider}
    class="w-fit" />
  ```
-->

<div class="mt-2 select-none {className}">
    <div class="label">
        <span class="label-text text-lg font-semibold mb-2">{label}</span>
    </div>
    <RangeSlider
        range
        bind:values
        {min}
        {max}
        on:stop={(e) => {
            if (onchange) onchange(e);
        }}
    />
    <div class="flex flex-row justify-between mt-6">
        <NumberInputSmall
            variant="primary"
            {min}
            {max}
            bind:value={values[0]}
            changeAmount={1}
            {onchange}
        />
        <NumberInputSmall
            variant="primary"
            {min}
            {max}
            bind:value={values[1]}
            changeAmount={10}
            {onchange}
        />
    </div>
    <div class="flex flex-row justify-between label">
        <span class="label-text text-pretty">Minimum</span>
        <span class="label-text text-pretty">Maximum</span>
    </div>
    {#if description}
        <div class="label">
            <span class="label-text text-pretty">{description}</span>
        </div>
    {/if}
</div>
