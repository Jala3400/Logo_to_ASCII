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
        processedGif,
        processedImageUrl,
        viewMode,
    } from "$lib/stores";
    import AsciiGifPlayer from "../molecules/AsciiGifPlayer.svelte";
    import GifPlayer from "../molecules/GifPlayer.svelte";

    let imageWrapper: HTMLElement | undefined = $state();
    let asciiWrapper: HTMLElement | undefined = $state();
    let isSyncing = false;

    function handleScroll(e: Event) {
        if (isSyncing || $viewMode !== "side-by-side") return;

        const source = e.target as HTMLElement;
        const target = source === imageWrapper ? asciiWrapper : imageWrapper;

        if (target) {
            isSyncing = true;
            target.scrollTop = source.scrollTop;
            target.scrollLeft = source.scrollLeft;
            requestAnimationFrame(() => {
                isSyncing = false;
            });
        }
    }

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

<div class="preview__split">
    <div class="preview__panel">
        <h3 class="preview__panel-title">
            {$imageDisplayMode === ImageDisplayMode.Original
                ? "Original"
                : "Processed"}
        </h3>

        <div
            bind:this={imageWrapper}
            onscroll={handleScroll}
            class="preview__scroll-area"
        >
            {#if $fileType === FileType.Gif}
                {#if gifAnimation}
                    <div class="preview__center-in-div">
                        <GifPlayer animation={gifAnimation} />
                    </div>
                {/if}
            {:else if $fileType === FileType.Image}
                {#if imageUrl}
                    <div class="preview__center-in-div">
                        <img
                            src={imageUrl}
                            alt={$imageDisplayMode === ImageDisplayMode.Original
                                ? "Original"
                                : "Processed Image"}
                            draggable="false"
                        />
                    </div>
                {/if}
            {/if}
        </div>
    </div>

    <div class="preview__panel">
        <h3 class="preview__panel-title">ASCII Output</h3>

        <div
            bind:this={asciiWrapper}
            onscroll={handleScroll}
            class="preview__scroll-area"
        >
            {#if $fileType === FileType.Gif}
                {#if $asciiGifOutput}
                    <div class="preview__center-in-div">
                        <AsciiGifPlayer animation={$asciiGifOutput} autoplay />
                    </div>
                {/if}
            {:else if $fileType === FileType.Image}
                {#if $asciiImageOutput}
                    <div class="preview__center-in-div">
                        {@html $asciiImageOutput}
                    </div>
                {/if}
            {/if}
        </div>
    </div>
</div>

<style>
    .preview__split {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: var(--spacing-md);
        height: 100%;
    }

    .preview__panel {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);
        min-width: 0;
        overflow: hidden;
    }

    .preview__panel-title {
        font-size: var(--font-xs);
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.05em;
        flex-shrink: 0;
    }

    /* Single scroll area — no flex centering, content overflows naturally */
    .preview__scroll-area {
        flex: 1;
        overflow: auto;
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        border: 1px solid var(--border);
        padding: var(--spacing-sm);
        line-height: 1;
        font-family: "Ubuntu Mono", monospace;
    }

    .preview__center-in-div {
        display: flex;
        align-items: center;
        justify-content: center;
        min-width: min-content;
        min-height: min-content;
        width: 100%;
        height: 100%;
    }

    .preview__scroll-area :global(pre) {
        margin: auto;
        white-space: pre;
        font-family: inherit;
        font-size: inherit;
        line-height: inherit;
    }
</style>
