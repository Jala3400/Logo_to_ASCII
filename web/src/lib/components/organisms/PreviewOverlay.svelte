<script lang="ts">
    import {
        asciiGifOutput,
        asciiImageOutput,
        config,
        FileType,
        fileType,
        ImageDisplayMode,
        imageDisplayMode,
        originalGif,
        originalImageUrl,
        overlayOpacity,
        processedGif,
        processedImageUrl,
    } from "$lib/stores";
    import AsciiGifPlayer from "../molecules/AsciiGifPlayer.svelte";
    import GifPlayer from "../molecules/GifPlayer.svelte";

    let imageUrl = $derived(
        $imageDisplayMode === ImageDisplayMode.Original
            ? $originalImageUrl
            : $processedImageUrl,
    );

    let gifAnimation = $derived(
        $imageDisplayMode === ImageDisplayMode.Original
            ? $originalGif
            : $processedGif,
    );
</script>

<div class="preview__overlay-container">
    <div class="preview__overlay-inner">
        {#if $fileType === FileType.Gif}
            {#if gifAnimation}
                <div
                    class="preview__image preview__overlay-base"
                    style="opacity: {$overlayOpacity}"
                >
                    <GifPlayer animation={gifAnimation} />
                </div>
            {/if}

            {#if $asciiGifOutput}
                <div class="preview__overlay-ascii">
                    <AsciiGifPlayer animation={$asciiGifOutput} autoplay />
                </div>
            {/if}
        {:else if $fileType === FileType.Image}
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

            {#if $asciiImageOutput}
                <div
                    class="preview__overlay-ascii"
                    style="font-size: {$config.char_size}px"
                >
                    {@html $asciiImageOutput}
                </div>
            {/if}
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

    /* Let gif-player render at natural size inside the overlay */
    .preview__overlay-base :global(gif-player)::part(img) {
        max-width: none;
        display: block;
    }

    /* Stretch ascii overlay to cover the base exactly */
    .preview__overlay-ascii {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: flex-start;
        justify-content: flex-start;
        line-height: 1;
        z-index: 2;
        font-family: "Ubuntu Mono", monospace;
    }
</style>
