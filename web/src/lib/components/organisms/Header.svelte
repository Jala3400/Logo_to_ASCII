<script lang="ts">
    interface Props {
        children?: import("svelte").Snippet;
    }

    let { children }: Props = $props();
</script>

<header class="header">
    <div class="header__left">
        <span class="header__logo">▓▒░</span>
        <h1 class="header__title">Logo to ASCII</h1>
    </div>
    <div class="header__right">
        {@render children?.()}
    </div>
</header>

<style>
    .header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        height: 3rem;
        padding: 0 var(--spacing-md);
        background-color: var(--bg-secondary);
        border-bottom: 1px solid var(--border);
        flex-shrink: 0;
    }

    .header__left {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
    }

    .header__logo {
        font-size: var(--font-lg);
        color: var(--accent);
        font-weight: 700;
    }

    .header__title {
        font-size: var(--font-base);
        font-weight: 600;
        color: var(--text-primary);
    }

    .header__right {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
    }
</style>
