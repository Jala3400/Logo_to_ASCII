<script lang="ts">
    import {
        asciiGifOutput,
        asciiImageOutput,
        config,
        FileType,
        fileType,
        processedGif,
        processedImageUrl,
    } from "$lib/stores";
    import { makeGif } from "$lib/utils/makeGif";
    import { OutputFormat } from "$lib/wasm";
    import Button from "../atoms/Button.svelte";

    let currentText = $derived(
        ($fileType === FileType.Image
            ? $asciiImageOutput
            : JSON.stringify($asciiGifOutput)) ??
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

    function downloadResult() {
        if (
            $config.format === OutputFormat.Html &&
            $fileType === FileType.Image
        ) {
            downloadHtml();
        } else {
            downloadTxt();
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
        switch ($fileType) {
            case FileType.Image:
                downloadProcessedImage();
                break;
            case FileType.Gif:
                downloadProcessedGif();
                break;
        }
    }

    async function downloadProcessedGif() {
        if ($processedGif) {
            const blob = await makeGif($processedGif);
            const a = document.createElement("a");
            a.href = URL.createObjectURL(blob);
            a.download = "output.gif";
            a.click();
        }
    }

    function downloadProcessedImage() {
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

    <Button
        size="sm"
        onclick={downloadResult}
        title="Download the result"
        disabled={$fileType === FileType.None}
    >
        💾 Save
    </Button>

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
