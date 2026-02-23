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
        onchange={(v) => viewMode.set(v as ViewMode)}
    />

    <Select
        label="Display image"
        value={imageModeStr}
        options={imageModeOptions}
        onchange={(v) => imageDisplayMode.set(v as ImageDisplayMode)}
    />

    {#if $viewMode === "overlay"}
        <div class="view-controls__slider">
            <Slider
                label="Image opacity"
                value={$overlayOpacity}
                min={0}
                max={1}
                step={0.05}
                onchange={(v) => overlayOpacity.set(v)}
            />
        </div>
    {/if}
</div>

<style>
    .view-controls {
        display: flex;
        flex-wrap: wrap;
        gap: var(--spacing-md);
        align-items: flex-end;
        flex: 1;
    }

    .view-controls__slider {
        flex: 1;
        min-width: 200px;
    }

    @media (max-width: 800px) {
        .view-controls__slider {
            flex-basis: 100%;
        }
    }
</style>
