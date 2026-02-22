<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        title: string;
        collapsed?: boolean;
        children?: Snippet;
    }

    let { title, collapsed = $bindable(false), children }: Props = $props();
</script>

<div class="section" class:section--collapsed={collapsed}>
    <button class="section__header" onclick={() => (collapsed = !collapsed)}>
        <span class="section__chevron">{collapsed ? "▸" : "▾"}</span>
        <span class="section__title">{title}</span>
    </button>

    {#if !collapsed}
        <div class="section__content">
            {@render children?.()}
        </div>
    {/if}
</div>

<style>
    .section {
        border-bottom: 1px solid var(--border);
    }

    .section__header {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
        width: 100%;
        padding: var(--spacing-sm) var(--spacing-md);
        background: none;
        border: none;
        color: var(--text-primary);
        font-family: inherit;
        font-size: var(--font-sm);
        font-weight: 600;
        cursor: pointer;
        transition: background-color var(--transition-base);
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .section__header:hover {
        background-color: var(--bg-tertiary);
    }

    .section__chevron {
        font-size: var(--font-xs);
        color: var(--text-muted);
        width: 1em;
    }

    .section__title {
        flex: 1;
        text-align: left;
    }

    .section__content {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);
        padding: var(--spacing-sm) var(--spacing-md) var(--spacing-md);
    }
</style>
