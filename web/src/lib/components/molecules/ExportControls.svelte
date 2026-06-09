<script lang="ts">
    import {
        asciiGifOutput,
        asciiImageOutput,
        config,
        FileType,
        fileType,
        processedImageUrl,
    } from "$lib/stores";
    import { OutputFormat } from "$lib/wasm";
    import Button from "../atoms/Button.svelte";

    let currentText = $derived(
        ($fileType === FileType.Image
            ? $asciiImageOutput
            : $asciiGifOutput?.toString()) ??
            "This should not happen: no ASCII output available.",
    );

    async function copyToClipboard() {
        try {
            await navigator.clipboard.writeText(currentText);
        } catch {
            // Fallback
            const el = document.createElement("textarea");
            el.value = currentText;
            document.body.appendChild(el);
            el.select();
            document.execCommand("copy");
            document.body.removeChild(el);
        }
    }

    function downloadTxt() {
        // Prepare text content: if it's HTML, we should really just get the text for a .txt file
        const blob = new Blob([currentText], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "ascii-art.txt";
        a.click();
        URL.revokeObjectURL(url);
    }

    function downloadHtml() {
        const blob = new Blob([currentText], { type: "text/html" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "ascii-art.html";
        a.click();
        URL.revokeObjectURL(url);
    }

    function downloadImage() {
        if ($processedImageUrl) {
            const a = document.createElement("a");
            a.href = $processedImageUrl;
            a.download = "ascii-art.png";
            a.click();
        }
    }
</script>

<div class="export">
    <Button
        size="sm"
        onclick={copyToClipboard}
        title="Copy ASCII to clipboard"
        disabled={$fileType === FileType.None}
    >
        📋 Copy
    </Button>

    {#if $config.format === OutputFormat.Ansi}
        <Button
            size="sm"
            onclick={downloadTxt}
            title="Download as .txt"
            disabled={$fileType === FileType.None}
        >
            💾 .txt
        </Button>
    {/if}

    {#if $config.format === OutputFormat.Html}
        <Button
            size="sm"
            onclick={downloadHtml}
            title="Download as .html"
            disabled={$fileType === FileType.None}
        >
            🌐 .html
        </Button>
    {/if}

    <Button
        size="sm"
        onclick={downloadImage}
        disabled={$fileType === FileType.None}
        title="Download processed image"
    >
        🖼️ {$fileType === FileType.Gif ? ".gif" : ".png"}
    </Button>
</div>

<style>
    .export {
        display: flex;
        gap: var(--spacing-sm);
        padding: var(--spacing-sm) var(--spacing-md);
        border-top: 1px solid var(--border);
        justify-content: space-around;
    }
</style>
