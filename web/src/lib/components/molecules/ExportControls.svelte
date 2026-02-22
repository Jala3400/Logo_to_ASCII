<script lang="ts">
    import { asciiOutput, processedImageUrl } from "$lib/stores";
    import Button from "../atoms/Button.svelte";

    async function copyToClipboard() {
        try {
            await navigator.clipboard.writeText($asciiOutput);
        } catch {
            // Fallback
            const el = document.createElement("textarea");
            el.value = $asciiOutput;
            document.body.appendChild(el);
            el.select();
            document.execCommand("copy");
            document.body.removeChild(el);
        }
    }

    function downloadTxt() {
        // Prepare text content: if it's HTML, we should really just get the text for a .txt file
        const blob = new Blob([$asciiOutput], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "ascii-art.txt";
        a.click();
        URL.revokeObjectURL(url);
    }

    function downloadHtml() {
        const blob = new Blob([$asciiOutput], { type: "text/html" });
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
    <Button size="sm" onclick={copyToClipboard} title="Copy ASCII to clipboard">
        📋 Copy
    </Button>

    <Button size="sm" onclick={downloadTxt} title="Download as .txt">
        💾 .txt
    </Button>

    <Button size="sm" onclick={downloadHtml} title="Download as .html">
        🌐 .html
    </Button>

    <Button
        size="sm"
        onclick={downloadImage}
        disabled={!$processedImageUrl}
        title="Download processed image"
    >
        🖼️ .png
    </Button>
</div>

<style>
    .export {
        display: flex;
        gap: var(--spacing-sm);
        padding: var(--spacing-sm) var(--spacing-md);
        border-top: 1px solid var(--border);
    }
</style>
