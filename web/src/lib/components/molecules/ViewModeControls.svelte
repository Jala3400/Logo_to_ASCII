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
    <Select
        label="View mode"
        value={viewModeStr}
        options={viewModeOptions}
        compact={true}
        onchange={(v) => viewMode.set(v as ViewMode)}
    />

    <Select
        label="Display image"
        value={imageModeStr}
        options={imageModeOptions}
        compact={true}
        onchange={(v) => imageDisplayMode.set(v as ImageDisplayMode)}
    />

    {#if $viewMode === "overlay"}
        <Slider
            label="Image opacity"
            value={$overlayOpacity}
            min={0}
            max={1}
            step={0.05}
            compact={true}
            oninput={(v) => overlayOpacity.set(v)}
        />
    {/if}
</div>

<style>
    .view-controls {
        display: flex;
        gap: var(--spacing-md);
        align-items: center;
        flex: 1;
    }
</style>
