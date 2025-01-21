<script lang="ts">
    import { invalidateAll } from "$app/navigation";
    import type { ChangeEventHandler } from "svelte/elements";

    let {
        selected = $bindable([]),
        examples = $bindable([]),
        label,
        description,
        onchange,
    }: {
        /**
         * *Bindable*, *Optional*
         *
         * Bindable string array of currently selected values.
         */
        selected?: string[];
        /**
         * *Bindable*, *Optional*
         *
         * Bindable string array of examples or suggestions that show up in a dropdown menu above the input field.
         */
        examples?: string[];
        /**
         * The label appearing above the input.
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
         * Callback that runs every time the value changes
         */
        onchange?: ChangeEventHandler<HTMLElement>;
    } = $props();

    let inputValue: string = $state("");
    let filteredExamples: string[] = $derived(
        examples.filter((v) => !selected.includes(v)),
    );

    function addToSelected(string: string, event: Event) {
        if (string !== "") {
            selected.push(string);
        }
        if (onchange) onchange(event);
    }

    function removeFromSelected(string: string, event: Event) {
        selected = selected.filter((selected) => selected !== string);
        if (onchange) onchange(event);
    }
</script>

<!--
@component
A multi selectable component with an input field and a dropdown menu that can be filled with examples.
The inputted items will be shown under the input field with a button to remove the items.

# Attributes:
- `selected` — Currently selected items. *Optional*, *Bindable*
- `examples` — Examples or suggestions that will show up above the input field. *Optional*, *Bindable*
- `label` — Label appearing above the input.
- `description` — Description appearing below the input. *Optional*
- `onchange` — Callback for when the value changes. *Optional*

# Usage:
```svelte
    <MultiSelect
        label="Label"
        description="Description stuff here."
        bind:selected={stringArray}
        examples={["Example One", "Example Two"]} />
-->

<div class="flex flex-col gap-2">
    <div class="text-lg font-semibold">{label}</div>
    <div class={filteredExamples.length > 0 ? "dropdown dropdown-top" : ""}>
        <div class="join">
            <input
                tabindex="0"
                type="text"
                bind:value={inputValue}
                onkeypress={(e) => {
                    if (e.key === "Enter") {
                        addToSelected(inputValue, e);
                        inputValue = "";
                    }
                }}
                class="join-item input-primary input focus:border-r-0 border-r-0 focus:outline-0 active:outline-none"
                placeholder="Application name"
            />
            <button
                onclick={(e) => {
                    addToSelected(inputValue, e);
                    inputValue = "";
                }}
                class="btn btn-neutral join-item border-l-0 no-animation border hover:border hover:border-l-0 hover:border-primary border-primary"
                >Add</button
            >
        </div>
        {#if filteredExamples.length > 0}
            <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
            <ul
                tabindex="0"
                class="menu dropdown-content bg-neutral rounded-box z-[1] min-w-40 p-2 mb-2 shadow"
            >
                {#each examples.filter((v) => !selected.includes(v)) as app, i}
                    <li>
                        <button onclick={(e) => addToSelected(app, e)}
                            >{app}</button
                        >
                    </li>
                {/each}
            </ul>
        {/if}
    </div>
    <div class="flex flex-wrap gap-1 max-w-sm">
        {#each selected as value, i}
            <div class="badge badge-lg badge-accent gap-1 pr-px">
                {value}
                <button
                    aria-label="Remove app"
                    class="btn btn-circle btn-ghost btn-xs"
                    onclick={(e) => removeFromSelected(value, e)}
                    ><svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="size-3"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M6 18 18 6M6 6l12 12"
                        />
                    </svg>
                </button>
            </div>
        {/each}
    </div>
    <div class="label pt-0">
        <span class="label-text text-pretty">{description}</span>
    </div>
</div>
