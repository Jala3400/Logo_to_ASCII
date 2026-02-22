import {
    imageBytes,
    config,
    asciiOutput,
    processedImageUrl,
    isConverting,
    errorMessage,
    wasmReady,
    originalImageUrl,
} from "./stores";
import { initWasm, convertImage, DEFAULT_CONFIG, type L2aConfig } from "./wasm";
import { get } from "svelte/store";

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

/**
 * Initialize the WASM module and mark it as ready.
 */
export async function initialize(): Promise<void> {
    try {
        await initWasm();
        wasmReady.set(true);
    } catch (e) {
        errorMessage.set(`Failed to initialize: ${e}`);
    }
}

/**
 * Load an image file. Creates the original preview URL and stores the bytes.
 */
export function loadImage(file: File): void {
    const reader = new FileReader();
    reader.onload = () => {
        const bytes = new Uint8Array(reader.result as ArrayBuffer);
        imageBytes.set(bytes);

        // Create preview URL for original image
        const blob = new Blob([bytes], { type: file.type });
        const url = URL.createObjectURL(blob);
        originalImageUrl.set(url);

        // Trigger conversion
        runConversion();
    };
    reader.readAsArrayBuffer(file);
}

/**
 * Run the conversion with debounce (avoids running on every keystroke).
 */
export function runConversion(delay = 150): void {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => doConvert(), delay);
}

async function doConvert(): Promise<void> {
    const bytes = get(imageBytes);
    if (!bytes) return;

    const cfg = get(config);

    isConverting.set(true);
    errorMessage.set(null);

    try {
        const result = await convertImage(bytes, cfg);

        asciiOutput.set(result.ascii);

        const prevUrl = get(processedImageUrl);
        if (prevUrl) URL.revokeObjectURL(prevUrl);
        processedImageUrl.set(result.imagePngUrl);
    } catch (e) {
        errorMessage.set(`Conversion failed: ${e}`);
    } finally {
        isConverting.set(false);
    }
}

/**
 * Update a single config field and re-run conversion.
 */
export function updateConfig<K extends keyof L2aConfig>(
    key: K,
    value: L2aConfig[K],
): void {
    config.update((cfg) => ({ ...cfg, [key]: value }));
    runConversion();
}

/**
 * Reset config to defaults.
 */
export function resetConfig(): void {
    config.set({ ...DEFAULT_CONFIG });
    runConversion();
}
