<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";
    import NumberInputSmall from "./NumberInputSmall.svelte";
    import type { HTMLAttributes } from "svelte/elements";

    let {
        label,
        description,
        class: className,
        min,
        max,
        values = $bindable(),
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
    <RangeSlider range bind:values {min} {max} />
    <div class="flex flex-row justify-between mt-6">
        <NumberInputSmall
            variant="primary"
            {min}
            {max}
            bind:value={values[0]}
        />
        <NumberInputSmall
            variant="primary"
            {min}
            {max}
            bind:value={values[1]}
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
