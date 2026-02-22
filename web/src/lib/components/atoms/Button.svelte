<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        variant?: "primary" | "secondary" | "ghost";
        size?: "sm" | "md";
        disabled?: boolean;
        title?: string;
        onclick?: (e: MouseEvent) => void;
        children?: Snippet;
    }

    let {
        variant = "secondary",
        size = "md",
        disabled = false,
        title = "",
        onclick,
        children,
    }: Props = $props();
</script>

<button class="btn btn--{variant} btn--{size}" {disabled} {title} {onclick}>
    {@render children?.()}
</button>

<style>
    .btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: var(--spacing-xs);
        border: 1px solid var(--border);
        border-radius: var(--radius-md);
        font-family: inherit;
        font-weight: 500;
        cursor: pointer;
        transition:
            background-color var(--transition-base),
            border-color var(--transition-base),
            color var(--transition-base);
        white-space: nowrap;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    /* Sizes */
    .btn--sm {
        padding: var(--spacing-xs) var(--spacing-sm);
        font-size: var(--font-xs);
    }

    .btn--md {
        padding: var(--spacing-sm) var(--spacing-md);
        font-size: var(--font-sm);
    }

    /* Variants */
    .btn--primary {
        background-color: var(--accent);
        border-color: var(--accent);
        color: var(--bg-primary);
    }

    .btn--primary:hover:not(:disabled) {
        background-color: var(--accent-hover);
        border-color: var(--accent-hover);
    }

    .btn--primary:active:not(:disabled) {
        background-color: var(--accent-active);
        border-color: var(--accent-active);
    }

    .btn--secondary {
        background-color: var(--bg-tertiary);
        border-color: var(--border);
        color: var(--text-primary);
    }

    .btn--secondary:hover:not(:disabled) {
        background-color: var(--border);
        border-color: var(--border-light);
    }

    .btn--ghost {
        background-color: transparent;
        border-color: transparent;
        color: var(--text-secondary);
    }

    .btn--ghost:hover:not(:disabled) {
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
    }
</style>
