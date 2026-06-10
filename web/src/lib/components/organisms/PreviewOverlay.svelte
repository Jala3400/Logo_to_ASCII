<script lang="ts">
    import {
        asciiImageOutput,
        config,
        FileType,
        fileType,
        ImageDisplayMode,
        imageDisplayMode,
        originalImageUrl,
        overlayOpacity,
        processedImageUrl,
    } from "$lib/stores";

    let imageUrl = $derived(
        $imageDisplayMode === ImageDisplayMode.Original
            ? $originalImageUrl
            : $processedImageUrl,
    );
</script>

<div class="preview__overlay-container">
    <div class="preview__overlay-inner">
        {#if $fileType === FileType.Gif}
            <p class="preview__overlay-placeholder">
                GIF preview not available.
            </p>
        {/if}

        {#if $fileType === FileType.Image}
            {#if imageUrl}
                <img
                    src={imageUrl}
                    alt={$imageDisplayMode === ImageDisplayMode.Original
                        ? "Original"
                        : "Processed Image"}
                    class="preview__image preview__overlay-base"
                    style="opacity: {$overlayOpacity}"
                    draggable="false"
                />
            {/if}

            <div
                class="preview__overlay-ascii"
                style="font-size: {$config.char_size}px"
            >
                {@html $asciiImageOutput}
            </div>
        {/if}
    </div>
</div>

<style>
    .preview__image {
        display: block;
    }

    .preview__overlay-container {
        position: relative;
        display: flex;
        height: 100%;
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        overflow: auto;
        align-items: center;
        justify-content: center;
    }

    .preview__overlay-inner {
        position: relative;
        display: inline-block;
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
        display: flex;
        align-items: flex-start;
        justify-content: flex-start;
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
