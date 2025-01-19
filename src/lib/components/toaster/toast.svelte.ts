import { createRandomString } from "$lib/utilities";
import type { MouseEventHandler } from "svelte/elements";

/**
 * A toast object with properties for the toast.
 */
type Toast = {
  /**
   * ID of the Toast
   */
  id: string;
  /**
   * Message of the toast
   */
  message: string;
  /**
   * The colour variant of the toast.
   */
  variant: "info" | "warning" | "success" | "error";
  /**
   * The button of the toast.
   */
  button?: {
    /**
     * The label of the toast button.
     */
    label: string;
    /**
     * The action that the toast will do when the button is clicked. It is typed out the same way as a `onclick` event on HTML Inputs.
     */
    action?: MouseEventHandler<HTMLButtonElement> | null | undefined;
  };
  /**
   * The amount of time the toast is shown for.
   */
  timeout: number | "persistent";
};

/**
 * Store that keeps track of the toasts.
 */
export const toasts = $state<Toast[]>([]);

/**
 * Remove a toast based on ID.
 *
 * @param id - The ID of the toast
 */
export function removeToast(id: string) {
  let i = toasts.findIndex((t) => t.id === id);
  if (i !== -1) {
    toasts.splice(i, 1);
  }
}

/**
 * Creates a new toast for the user.
 * @param message - The message to be displayed in the toast.
 * @param toastProps - Optional properties of the toast.
 * @returns id - A string of the toasts ID.
 */
export function toast(
  message: string,
  toastProps: Partial<Omit<Toast, "id" | "message">> = {
    variant: "info",
    timeout: 10,
  },
) {
  const newToast = () => {
    let newToast: Toast = {
      id: createRandomString(5),
      message: message,
      timeout: toastProps.timeout ?? 10,
      variant: toastProps.variant ?? "info",
    };
    if (toastProps.button) {
      newToast.button = {
        label: toastProps.button.label,
        action: toastProps.button.action,
      };
    }
    return newToast;
  };
  toasts.push(newToast());
  if (toastProps.timeout !== "persistent") {
    setTimeout(
      () => {
        removeToast(newToast().id);
      },
      Number(newToast().timeout) * 1000,
    );
  }
  return newToast().id;
}
