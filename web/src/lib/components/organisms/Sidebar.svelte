<script lang="ts">
    import { sidebarCollapsed } from "$lib/stores";
    import { runConversion } from "$lib/converter";
    import { config } from "$lib/stores";
    import { DEFAULT_CONFIG } from "$lib/wasm";
    import Button from "../atoms/Button.svelte";
    import AlgorithmControls from "../molecules/AlgorithmControls.svelte";
    import BorderControls from "../molecules/BorderControls.svelte";
    import CharacterControls from "../molecules/CharacterControls.svelte";
    import ExportControls from "../molecules/ExportControls.svelte";
    import ImageControls from "../molecules/ImageControls.svelte";
    import SizeControls from "../molecules/SizeControls.svelte";

    function resetConfig() {
        config.set({ ...DEFAULT_CONFIG });
        runConversion();
    }
</script>

<aside class="sidebar" class:sidebar--collapsed={$sidebarCollapsed}>
    <div class="sidebar__scroll">
        <div class="sidebar__controls">
            <CharacterControls />
            <ImageControls />
            <BorderControls />
            <SizeControls />
            <AlgorithmControls />
        </div>

        <div class="sidebar__actions">
            <Button size="sm" variant="ghost" onclick={resetConfig}>
                ↺ Reset defaults
            </Button>
        </div>

        <ExportControls />
    </div>
</aside>

<style>
    .sidebar {
        width: var(--sidebar-width);
        min-width: var(--sidebar-width);
        height: 100%;
        background-color: var(--bg-secondary);
        border-right: 1px solid var(--border);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        transition: all 0.2s ease-in-out;
    }

    .sidebar--collapsed {
        width: 0;
        min-width: 0;
        border-right-width: 0;
        pointer-events: none;
    }

    .sidebar__scroll {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        min-width: var(--sidebar-width);
    }

    .sidebar__controls {
        flex: 1;
    }

    .sidebar__actions {
        display: flex;
        justify-content: center;
        padding: var(--spacing-sm) var(--spacing-md);
        border-top: 1px solid var(--border);
    }
</style>
