<script lang="ts">
    import { fly } from "svelte/transition";
    import { removeToast, toasts } from "./toast.svelte";
    import { flip } from "svelte/animate";
</script>

<div class="toast toast-top toast-end">
    {#each toasts as toast (toast.id)}
        <div
            animate:flip={{ duration: 500 }}
            out:fly={{ y: -100, duration: 500 }}
            class="w-64 p-0 alert flex flex-col items-start gap-2
            {toast.variant === 'info'
                ? 'alert-info'
                : toast.variant === 'error'
                  ? 'alert-error'
                  : toast.variant === 'success'
                    ? 'alert-success'
                    : toast.variant === 'warning'
                      ? 'alert-warning'
                      : 'alert-info'}">
            <button
                aria-label="Close toast"
                onclick={() => {
                    removeToast(toast.id);
                }}
                class="btn btn-sm btn-circle btn-ghost self-end"
                ><svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="size-3">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M6 18 18 6M6 6l12 12" />
                </svg></button>
            <div class=" {toast.button ? 'px-4' : ' p-4 pt-0'} text-wrap">
                {toast.message}
            </div>
            {#if toast.button}
                <button
                    onclick={toast.button.action}
                    class="btn btn-sm m-4 mt-0 self-end
                    {toast.variant === 'info'
                        ? 'btn-info'
                        : toast.variant === 'error'
                          ? 'btn-error'
                          : toast.variant === 'success'
                            ? 'btn-success'
                            : toast.variant === 'warning'
                              ? 'btn-warning'
                              : 'btn-info'}">{toast.button.label}</button>
            {/if}
        </div>
    {/each}
</div>
