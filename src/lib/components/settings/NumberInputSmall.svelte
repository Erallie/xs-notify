<script lang="ts">
    import type { ChangeEventHandler, HTMLAttributes } from "svelte/elements";

    let {
        value = $bindable(0),
        onchange,
        min,
        max,
        label,
        description,
        variant,
        changeAmount = 1,
        class: className,
    }: {
        /**
         * *Optional*, *Bindable*
         *
         * The value of the input
         */
        value?: number;
        /**
         * *Optional*
         *
         * Callback that runs every time the value changes
         */
        onchange?: ChangeEventHandler<HTMLInputElement>;
        /**
         * *Optional*
         *
         * The minimum amount the value can be set
         */
        min?: number;
        /**
         * *Optional*
         *
         * The max amount a value can be set
         */
        max?: number;
        /**
         * *Optional*
         *
         * The label appearing above the input.
         */
        label?: string;
        /**
         * *Optional*
         *
         *  Sets the description label under the input
         */
        description?: string;
        /**
         * *Optional*
         *
         * Sets the button variant
         */
        variant?:
            | "primary"
            | "secondary"
            | "accent"
            | "info"
            | "success"
            | "warning"
            | "error";
        /**
         * *Optional*
         *
         * The amount the value will change when clicking on the buttons.
         */
        changeAmount: number;
        /**
         * *Optional*
         *
         * Additional classes that can be put on the wrapping div element
         */
        class?: HTMLAttributes<HTMLDivElement>["class"];
    } = $props();
</script>

<!--
@component
Smaller number input with buttons to change the value.

# Properties
- `label` — Label appearing above the input. *Optional*
- `description` — Description appearing below the input. *Optional*
- `value` — The value of the input. *Bindable*, *Optional*
- `variant` — The variant of the input and its buttons. *Optional*
- `class` — Additional classes to apply to the wrapper element. *Optional*
- `min` — The minimum value of the input. *Optional*
- `max` — The maximum value of the input. *Optional*
- `onchange` — Callback for when the value changes. *Optional*
- `changeAmount` — The amount the value will change when clicking on the buttons. *Optional*

# Usage:
```svelte
  <NumberInputBig
    label="Input label"
    description="Input description"
    bind:value={value}
    variant="accent"
    min={10}
    max={90}
    onchange={functionName()} />
```
-->

<div>
    {#if label}
        <div class="text-lg font-semibold mb-2">{label}</div>
    {/if}
    <div
        class="flex flex-row rounded-btn border items-center input-bordered max-w-fit text-lg outline-2 outline-offset-2 peer-focus:outline focus-within:outline focus-within:border focus:border
        {variant === 'primary'
            ? 'input-primary'
            : variant === 'secondary'
              ? 'input-secondary'
              : variant === 'accent'
                ? 'input-accent'
                : variant === 'info'
                  ? 'input-info'
                  : variant === 'success'
                    ? 'input-success'
                    : variant === 'warning'
                      ? 'input-warning'
                      : variant === 'error'
                        ? 'input-error'
                        : ''}"
    >
        <input
            type="number"
            {min}
            {max}
            {onchange}
            class="input input-sm peer max-w-12 border-none focus:outline-none focus-within:outline-none focus:border-none [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            bind:value
        />
        <div class="flex flex-col w-8 gap-px p-px">
            <button
                onclick={() => {
                    if (value !== max) value = value + changeAmount;
                }}
                aria-label="Increase number"
                class="btn btn-xs p-0 min-h-4 max-h-4 rounded-b-none
                {variant === 'primary'
                    ? 'btn-primary'
                    : variant === 'secondary'
                      ? 'btn-secondary'
                      : variant === 'accent'
                        ? 'btn-accent'
                        : variant === 'info'
                          ? 'btn-info'
                          : variant === 'success'
                            ? 'btn-success'
                            : variant === 'warning'
                              ? 'btn-warning'
                              : variant === 'error'
                                ? 'btn-error'
                                : ''}"
                ><svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    class="size-3"
                >
                    <path
                        fill-rule="evenodd"
                        d="M11.47 7.72a.75.75 0 0 1 1.06 0l7.5 7.5a.75.75 0 1 1-1.06 1.06L12 9.31l-6.97 6.97a.75.75 0 0 1-1.06-1.06l7.5-7.5Z"
                        clip-rule="evenodd"
                    />
                </svg></button
            >
            <button
                onclick={() => {
                    if (value !== min) value = value - changeAmount;
                }}
                aria-label="Decrease number"
                class="btn btn-xs p-0 min-h-4 max-h-4 rounded-t-none
                {variant === 'primary'
                    ? 'btn-primary'
                    : variant === 'secondary'
                      ? 'btn-secondary'
                      : variant === 'accent'
                        ? 'btn-accent'
                        : variant === 'info'
                          ? 'btn-info'
                          : variant === 'success'
                            ? 'btn-success'
                            : variant === 'warning'
                              ? 'btn-warning'
                              : variant === 'error'
                                ? 'btn-error'
                                : ''}"
                ><svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    class="size-3"
                >
                    <path
                        fill-rule="evenodd"
                        d="M12.53 16.28a.75.75 0 0 1-1.06 0l-7.5-7.5a.75.75 0 0 1 1.06-1.06L12 14.69l6.97-6.97a.75.75 0 1 1 1.06 1.06l-7.5 7.5Z"
                        clip-rule="evenodd"
                    />
                </svg></button
            >
        </div>
    </div>
    {#if description}
        <div class="label">
            <span class="label-text text-pretty">{description}</span>
        </div>
    {/if}
</div>
