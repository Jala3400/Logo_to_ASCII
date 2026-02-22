<script lang="ts">
    import CharacterControls from "../molecules/CharacterControls.svelte";
    import ImageControls from "../molecules/ImageControls.svelte";
    import BorderControls from "../molecules/BorderControls.svelte";
    import SizeControls from "../molecules/SizeControls.svelte";
    import AlgorithmControls from "../molecules/AlgorithmControls.svelte";
    import ExportControls from "../molecules/ExportControls.svelte";
    import Button from "../atoms/Button.svelte";
    import { hasImage, wasmReady } from "$lib/stores";
    import { DEFAULT_CONFIG } from "$lib/wasm";
    import { config } from "$lib/stores";
    import { runConversion } from "$lib/converter";

    function resetConfig() {
        config.set({ ...DEFAULT_CONFIG });
        runConversion();
    }
</script>

<aside class="sidebar">
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
    }

    .sidebar__scroll {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
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
