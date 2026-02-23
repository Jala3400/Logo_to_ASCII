<script lang="ts">
    import {
        asciiOutput,
        config,
        imageDisplayMode,
        originalImageUrl,
        processedImageUrl,
        viewMode,
    } from "$lib/stores";

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
</script>

<div class="preview__split">
    <div class="preview__panel">
        <h3 class="preview__panel-title">
            {$imageDisplayMode === "original" ? "Original" : "Processed Image"}
        </h3>
        <div
            bind:this={imageWrapper}
            onscroll={handleScroll}
            class="preview__image-wrapper"
        >
            {#if $imageDisplayMode === "original"}
                {#if $originalImageUrl}
                    <img
                        src={$originalImageUrl}
                        alt="Original"
                        class="preview__image"
                        draggable="false"
                    />
                {/if}
            {:else if $processedImageUrl}
                <img
                    src={$processedImageUrl}
                    alt="Processed"
                    class="preview__image"
                    draggable="false"
                />
            {/if}
        </div>
    </div>
    <div class="preview__panel">
        <h3 class="preview__panel-title">ASCII Output</h3>
        <div
            bind:this={asciiWrapper}
            onscroll={handleScroll}
            class="preview__ascii-wrapper"
            style="font-size: {$config.char_size}px"
        >
            {@html $asciiOutput}
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

    .preview__image-wrapper {
        flex: 1;
        display: flex;
        overflow: auto;
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        border: 1px solid var(--border);
    }

    .preview__image {
        display: block;
        margin: auto;
    }

    .preview__ascii-wrapper {
        flex: 1;
        overflow: auto;
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        border: 1px solid var(--border);
        padding: var(--spacing-sm);
        line-height: 1;
        font-family: "Ubuntu Mono", monospace;
        display: flex;
    }

    .preview__ascii-wrapper :global(pre) {
        margin: auto;
        white-space: pre;
        font-family: inherit;
        font-size: inherit;
        line-height: inherit;
    }
</style>
