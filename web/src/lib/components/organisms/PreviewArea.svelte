<script lang="ts">
    import { loadImage } from "$lib/converter";
    import {
        asciiOutput,
        config,
        errorMessage,
        hasImage,
        imageDisplayMode,
        isConverting,
        originalImageUrl,
        overlayOpacity,
        processedImageUrl,
        viewMode,
        wasmReady,
    } from "$lib/stores";
    import ViewModeControls from "../molecules/ViewModeControls.svelte";

    let imageUrl = $derived(
        $imageDisplayMode === "original"
            ? $originalImageUrl
            : $processedImageUrl,
    );

    let dragover = $state(false);
    let fileInput: HTMLInputElement;

    function handleDrop(e: DragEvent) {
        e.preventDefault();
        dragover = false;
        if (!$wasmReady) return;
        const file = e.dataTransfer?.files[0];
        if (file && file.type.startsWith("image/")) {
            loadImage(file);
        }
    }

    function handleDragOver(e: DragEvent) {
        e.preventDefault();
        dragover = true;
    }

    function handleDragLeave() {
        dragover = false;
    }

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

<main
    class="preview"
    class:preview--dragover={dragover}
    ondrop={handleDrop}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
>
    {#if !$wasmReady}
        <div class="preview__empty">
            <span class="preview__spinner">⏳</span>
            <p>Loading WASM module...</p>
        </div>
    {:else if !$hasImage}
        <div
            class="preview__empty"
            onclick={openFilePicker}
            role="button"
            tabindex="0"
            onkeydown={(e) => e.key === "Enter" && openFilePicker()}
        >
            <span class="preview__empty-icon">🖼️</span>
            <p class="preview__empty-title">Drop an image here</p>
            <p class="preview__empty-hint">or click to browse</p>
        </div>
    {:else}
        <div class="preview__toolbar">
            <ViewModeControls />
            {#if $isConverting}
                <span class="preview__status">Converting...</span>
            {/if}
            <button class="preview__change-img" onclick={openFilePicker}>
                Change Image
            </button>
        </div>

        {#if $errorMessage}
            <div class="preview__error">
                <span>⚠️</span>
                <span>{$errorMessage}</span>
            </div>
        {/if}

        <div class="preview__content">
            {#if $viewMode === "side-by-side"}
                <div class="preview__split">
                    <div class="preview__panel">
                        <h3 class="preview__panel-title">
                            {$imageDisplayMode === "original"
                                ? "Original"
                                : "Processed Image"}
                        </h3>
                        <div class="preview__image-wrapper">
                            {#if $imageDisplayMode === "original"}
                                {#if $originalImageUrl}
                                    <img
                                        src={$originalImageUrl}
                                        alt="Original"
                                        class="preview__image"
                                    />
                                {/if}
                            {:else if $processedImageUrl}
                                <img
                                    src={$processedImageUrl}
                                    alt="Processed"
                                    class="preview__image"
                                />
                            {/if}
                        </div>
                    </div>
                    <div class="preview__panel">
                        <h3 class="preview__panel-title">ASCII Output</h3>
                        <div
                            class="preview__ascii-wrapper"
                            style="font-size: {$config.char_size}px"
                        >
                            {@html $asciiOutput}
                        </div>
                    </div>
                </div>
            {:else if $viewMode === "overlay"}
                <div class="preview__overlay-container">
                    {#if imageUrl}
                        <img
                            src={imageUrl}
                            alt="Base"
                            class="preview__image preview__overlay-base"
                            style="opacity: {$overlayOpacity}"
                        />
                    {/if}
                    <div
                        class="preview__overlay-ascii"
                        style="font-size: {$config.char_size}px"
                    >
                        {@html $asciiOutput}
                    </div>
                </div>
            {/if}
        </div>
    {/if}

    <input
        type="file"
        accept="image/*"
        bind:this={fileInput}
        onchange={handleFileSelect}
        style="display: none;"
    />
</main>

<style>
    .preview {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        background-color: var(--bg-primary);
        position: relative;
        transition: border-color 0.2s ease;
        border: 2px solid transparent;
    }

    .preview--dragover {
        border-color: var(--accent);
        background-color: rgba(0, 255, 170, 0.05); /* Assuming teal accent */
    }

    /* Empty state */
    .preview__empty {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: var(--spacing-sm);
        color: var(--text-muted);
        cursor: pointer;
        padding: var(--spacing-xl);
        border: 2px dashed transparent;
        transition: all 0.2s ease;
    }

    .preview:not(.preview--dragover) .preview__empty:hover {
        border-color: var(--border);
        background-color: rgba(255, 255, 255, 0.02);
    }

    .preview__empty-icon {
        font-size: 3rem;
        opacity: 0.5;
    }

    .preview__spinner {
        font-size: 2rem;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    .preview__empty-title {
        font-size: var(--font-lg);
        color: var(--text-secondary);
    }

    .preview__empty-hint {
        font-size: var(--font-sm);
    }

    /* Toolbar */
    .preview__toolbar {
        display: flex;
        align-items: flex-end;
        gap: var(--spacing-md);
        padding: var(--spacing-sm);
        border-bottom: 1px solid var(--border);
        background-color: var(--bg-secondary);
        flex-shrink: 0;
    }

    .preview__status {
        font-size: var(--font-xs);
        color: var(--accent);
        animation: pulse 1s ease-in-out infinite;
    }

    .preview__change-img {
        margin-left: auto;
        background: transparent;
        border: 1px solid var(--border);
        color: var(--text-muted);
        padding: 0.25rem 0.75rem;
        border-radius: var(--radius-sm);
        font-size: var(--font-xs);
        cursor: pointer;
        transition: all 0.2s;
    }

    .preview__change-img:hover {
        border-color: var(--accent);
        color: var(--accent);
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

    /* Error */
    .preview__error {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
        padding: var(--spacing-sm) var(--spacing-md);
        background-color: oklch(20% 0.05 20);
        color: var(--error);
        font-size: var(--font-sm);
        border-bottom: 1px solid var(--error);
    }

    /* Content area */
    .preview__content {
        flex: 1;
        overflow: auto;
        padding: var(--spacing-md);
    }

    /* Side by side */
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
        align-items: center;
        justify-content: center;
        overflow: auto;
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        border: 1px solid var(--border);
    }

    .preview__image {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;
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
        align-items: center;
        justify-content: center;
    }

    /* Overlay mode */
    .preview__overlay-container {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        overflow: hidden;
    }

    .preview__overlay-base {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;
        z-index: 1;
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
        pointer-events: none;
        z-index: 2;
        font-family: "Ubuntu Mono", monospace;
    }

    /* Ensure pre tags inside the ASCII output behave properly */
    .preview__ascii-wrapper :global(pre),
    .preview__overlay-ascii :global(pre) {
        margin: 0;
        white-space: pre;
        font-family: inherit;
        font-size: inherit;
        line-height: inherit;
    }
</style>
