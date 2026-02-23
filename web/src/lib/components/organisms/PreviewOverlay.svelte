<script lang="ts">
    import {
        asciiOutput,
        config,
        imageDisplayMode,
        originalImageUrl,
        overlayOpacity,
        processedImageUrl,
    } from "$lib/stores";

    let imageUrl = $derived(
        $imageDisplayMode === "original"
            ? $originalImageUrl
            : $processedImageUrl,
    );
</script>

<div class="preview__overlay-container">
    <div class="preview__overlay-inner">
        {#if imageUrl}
            <img
                src={imageUrl}
                alt="Base"
                class="preview__image preview__overlay-base"
                style="opacity: {$overlayOpacity}"
                draggable="false"
            />
        {/if}
        <div
            class="preview__overlay-ascii"
            style="font-size: {$config.char_size}px"
        >
            {@html $asciiOutput}
        </div>
    </div>
</div>

<style>
    .preview__image {
        display: block;
        margin: auto;
    }

    .preview__overlay-container {
        position: relative;
        display: flex;
        height: 100%;
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        overflow: auto;
    }

    .preview__overlay-inner {
        position: relative;
        margin: auto;
    }

    .preview__overlay-base {
        display: block;
        z-index: 1;
        pointer-events: none;
    }

    .preview__overlay-ascii {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        line-height: 1;
        z-index: 2;
        font-family: "Ubuntu Mono", monospace;
    }

    .preview__overlay-ascii :global(pre) {
        margin: auto;
        white-space: pre;
        font-family: inherit;
        font-size: inherit;
        line-height: inherit;
    }
</style>
