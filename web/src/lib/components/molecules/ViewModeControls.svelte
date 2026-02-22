<script lang="ts">
    import {
        imageDisplayMode,
        ImageDisplayMode,
        overlayOpacity,
        viewMode,
        ViewMode,
    } from "$lib/stores";
    import Select from "../atoms/Select.svelte";
    import Slider from "../atoms/Slider.svelte";

    const viewModeOptions = [
        { value: "side-by-side", label: "Side by Side" },
        { value: "overlay", label: "Overlay" },
    ];

    const imageModeOptions = [
        { value: "original", label: "Original" },
        { value: "processed", label: "Output image" },
    ];

    let viewModeStr = $derived($viewMode as string);
    let imageModeStr = $derived($imageDisplayMode as string);
</script>

<div class="view-controls">
    <div class="view-controls__row">
        <Select
            label="View mode"
            value={viewModeStr}
            options={viewModeOptions}
            onchange={(v) => viewMode.set(v as ViewMode)}
        />

        <Select
            label="Display image"
            value={imageModeStr}
            options={imageModeOptions}
            onchange={(v) => imageDisplayMode.set(v as ImageDisplayMode)}
        />
    </div>
    {#if $viewMode === "overlay"}
        <Slider
            label="Image opacity"
            value={$overlayOpacity}
            min={0}
            max={1}
            step={0.05}
            onchange={(v) => overlayOpacity.set(v)}
        />
    {/if}
</div>

<style>
    .view-controls {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);
        padding: var(--spacing-sm) var(--spacing-md);
        border-bottom: 1px solid var(--border);
        width: 100%;
    }

    .view-controls__row {
        display: flex;
        gap: var(--spacing-md);
        align-items: flex-end;
    }
</style>
