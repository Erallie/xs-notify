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
         * The label appearing above the input.
         */
        label: string;
        /**
         * Sets the description label under the input
         */
        description: string;
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
        changeAmount?: number;
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
Number input with buttons to change the value.

# Properties
- `label` — Label appearing above the input.
- `description` — Description appearing below the input.
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
    class="bg-base-300 rounded-box"
    min={10}
    max={90}
    onchange={functionName()} />
```
-->

<div class={className}>
    <div class="text-lg font-semibold mb-2">{label}</div>
    <div
        class="flex flex-row rounded-btn border item-center input-bordered w-fit text-lg outline-2 outline-offset-2 peer-focus:outline focus-within:outline focus-within:border focus:border
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
        <button
            onclick={() => {
                if (value !== max) value = value - changeAmount;
            }}
            aria-label="Decrease number"
            class="btn text-lg rounded-r-none {variant === 'primary'
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
                class="size-4"
            >
                <path
                    fill-rule="evenodd"
                    d="M4.25 12a.75.75 0 0 1 .75-.75h14a.75.75 0 0 1 0 1.5H5a.75.75 0 0 1-.75-.75Z"
                    clip-rule="evenodd"
                />
            </svg></button
        >
        <input
            type="number"
            {max}
            {min}
            {onchange}
            class="input input-ghost peer max-w-20 border-none focus:outline-none focus-within:outline-none focus:border-none [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            bind:value
        />
        <button
            aria-label="Increase number"
            class="btn text-lg rounded-l-none {variant === 'primary'
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
            onclick={() => {
                if (value !== max) value = value + changeAmount;
            }}
            ><svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="size-4"
            >
                <path
                    fill-rule="evenodd"
                    d="M12 3.75a.75.75 0 0 1 .75.75v6.75h6.75a.75.75 0 0 1 0 1.5h-6.75v6.75a.75.75 0 0 1-1.5 0v-6.75H4.5a.75.75 0 0 1 0-1.5h6.75V4.5a.75.75 0 0 1 .75-.75Z"
                    clip-rule="evenodd"
                />
            </svg></button
        >
    </div>
    <div class="label">
        <span class="label-text text-pretty">{description}</span>
    </div>
</div>
