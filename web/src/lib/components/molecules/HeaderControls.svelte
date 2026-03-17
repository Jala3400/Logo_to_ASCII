<script lang="ts">
    import Button from "$lib/components/atoms/Button.svelte";
    import { loadImage } from "$lib/converter";
    import { hasImage, isConverting, wasmReady } from "$lib/stores";
    import ViewModeControls from "./ViewModeControls.svelte";

    let fileInput: HTMLInputElement;

    function handleFileSelect(e: Event) {
        const target = e.target as HTMLInputElement;
        const file = target.files?.[0];
        if (file) {
            loadImage(file);
        }
        target.value = "";
    }

    function openFilePicker() {
        if ($wasmReady) fileInput.click();
    }
</script>

<div class="header-controls">
    {#if $hasImage}
        <div class="header-controls__content">
            <ViewModeControls />
        </div>
        <div class="change-img-btn-wrapper">
            <Button
                size="sm"
                variant="ghost"
                onclick={openFilePicker}
            >
                Change Image
            </Button>
        </div>
    {/if}

    {#if $isConverting}
        <span class="header-controls__status">Converting...</span>
    {/if}

    <input
        type="file"
        accept="image/*"
        bind:this={fileInput}
        onchange={handleFileSelect}
        style="display: none;"
    />
</div>

<style>
    .header-controls {
        display: flex;
        align-items: center;
        gap: var(--spacing-md);
        flex: 1;
    }

    .header-controls__content {
        display: flex;
        align-items: center;
        gap: var(--spacing-md);
        flex: 1;
    }

    .header-controls__status {
        font-size: var(--font-xs);
        color: var(--accent);
        animation: pulse 1s ease-in-out infinite;
        flex-shrink: 0;
    }

    .change-img-btn-wrapper {
        flex-shrink: 0;
    }

    @keyframes pulse {
        0%,
        100% {
            opacity: 1;
        }
        50% {
            opacity: 0.5;
        }
    }
</style>
