import { get } from "svelte/store";
import {
    asciiGifOutput,
    asciiImageOutput,
    config,
    errorMessage,
    FileType,
    fileType,
    gifBytes,
    imageBytes,
    isConverting,
    originalGif,
    originalImageUrl,
    processedGif,
    processedImageUrl,
    wasmReady,
} from "./stores";
import {
    convertGif,
    convertImage,
    DEFAULT_CONFIG,
    initWasm,
    type L2aConfig,
} from "./wasm";

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

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

export function loadFile(file: File): void {
    if (file.type === "image/gif") {
        loadGif(file);
    } else if (file.type.startsWith("image/")) {
        loadImage(file);
    }
}

/**
 * Load an image file. Creates the original preview URL and stores the bytes.
 */
function loadImage(file: File): void {
    const reader = new FileReader();
    reader.onload = () => {
        const bytes = new Uint8Array(reader.result as ArrayBuffer);
        imageBytes.set(bytes);

        const prevOriginalUrl = get(originalImageUrl);
        if (prevOriginalUrl) {
            URL.revokeObjectURL(prevOriginalUrl);
        }
        // Create preview URL for original image
        const blob = new Blob([bytes], { type: file.type });
        const url = URL.createObjectURL(blob);
        originalImageUrl.set(url);

        // Trigger conversion
        runConversion();
    };
    reader.readAsArrayBuffer(file);

    fileType.set(FileType.Image);
}

/**
 * Load a GIF file. Calls
 */
function loadGif(file: File): void {
    const reader = new FileReader();
    reader.onload = () => {
        const bytes = new Uint8Array(reader.result as ArrayBuffer);
        gifBytes.set(bytes);

        // Trigger conversion
        runConversion();
    };
    reader.readAsArrayBuffer(file);

    fileType.set(FileType.Gif);
}

/**
 * Run the conversion with debounce (avoids running on every keystroke).
 */
export function runConversion(delay = 150): void {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => doConvert(), delay);
}

async function doConvert(): Promise<void> {
    const type = get(fileType);

    isConverting.set(true);
    errorMessage.set(null);

    try {
        if (type === FileType.Image) {
            await doConvertImage();
        } else if (type === FileType.Gif) {
            await doConvertGif();
        }
    } catch (e) {
        errorMessage.set(`Conversion failed: ${e}`);
        fileType.set(FileType.None);
        clearAllData();
    } finally {
        isConverting.set(false);
    }
}

async function doConvertImage(): Promise<void> {
    const bytes = get(imageBytes);
    if (!bytes) return;

    const cfg = get(config);

    const result = await convertImage(bytes, cfg);

    // Clear previous processed image URL to avoid memory leaks
    const prevProcessedImgUrl = get(processedImageUrl);
    if (prevProcessedImgUrl) URL.revokeObjectURL(prevProcessedImgUrl);

    asciiImageOutput.set(result.ascii);
    processedImageUrl.set(result.imagePngUrl);
}

async function doConvertGif(): Promise<void> {
    const bytes = get(gifBytes);
    if (!bytes) return;

    const cfg = get(config);

    const result = await convertGif(bytes, cfg);

    // Clear previous processed GIF URLs to avoid memory leaks
    clearGifUrls();

    asciiGifOutput.set(result.ascii_json);
    originalGif.set(result.originalGif);
    processedGif.set(result.processedGif);
}

function clearGifUrls(): void {
    const originalFrames = get(originalGif);
    if (originalFrames) {
        for (const frame of originalFrames) {
            URL.revokeObjectURL(frame.pngUrl);
        }
    }

    const processedFrames = get(processedGif);
    if (processedFrames) {
        for (const frame of processedFrames) {
            URL.revokeObjectURL(frame.pngUrl);
        }
    }
}

function clearAllData(): void {
    const originalImgUrl = get(originalImageUrl);
    if (originalImgUrl) URL.revokeObjectURL(originalImgUrl);
    const processedImgUrl = get(processedImageUrl);
    if (processedImgUrl) URL.revokeObjectURL(processedImgUrl);
    clearGifUrls();

    imageBytes.set(null);
    gifBytes.set(null);
    originalImageUrl.set(null);
    processedImageUrl.set(null);
    originalGif.set(null);
    processedGif.set(null);
    asciiImageOutput.set(null);
    asciiGifOutput.set(null);
}
