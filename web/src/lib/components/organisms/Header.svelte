<script lang="ts">
    import Button from "$lib/components/atoms/Button.svelte";
    import { sidebarCollapsed } from "$lib/stores";

    interface Props {
        children?: import("svelte").Snippet;
    }

    let { children }: Props = $props();

    function toggleSidebar() {
        sidebarCollapsed.update((v) => !v);
    }
</script>

<header class="header">
    <div class="header__left">
        <Button
            size="sm"
            variant="ghost"
            onclick={toggleSidebar}
            aria-label="Toggle Sidebar"
        >
            <div class="header__toggle-icon">
                <span></span>
                <span></span>
                <span></span>
            </div>
        </Button>
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

    /* Toggle Icon */
    .header__toggle-icon {
        display: flex;
        flex-direction: column;
        gap: 3px;
        width: 18px;
    }

    .header__toggle-icon span {
        display: block;
        width: 100%;
        height: 2px;
        background-color: currentColor;
        border-radius: 1px;
    }
</style>
