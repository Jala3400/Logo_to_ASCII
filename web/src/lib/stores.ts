import { writable, derived } from "svelte/store";
import { DEFAULT_CONFIG, type L2aConfig } from "./wasm";

/** Currently loaded image as raw bytes */
export const imageBytes = writable<Uint8Array | null>(null);

/** Original image object URL for preview */
export const originalImageUrl = writable<string | null>(null);

/** User configuration (reactive — triggers re-conversion) */
export const config = writable<L2aConfig>({ ...DEFAULT_CONFIG });

/** ASCII output HTML */
export const asciiOutput = writable<string>("");

/** Processed image blob URL */
export const processedImageUrl = writable<string | null>(null);

/** Whether a conversion is currently running */
export const isConverting = writable<boolean>(false);

/** Error message from last conversion, if any */
export const errorMessage = writable<string | null>(null);

/** Whether the WASM module has been initialized */
export const wasmReady = writable<boolean>(false);

/** Current preview view mode */
export enum ViewMode {
    SideBySide = "side-by-side",
    Overlay = "overlay",
}
export const viewMode = writable<ViewMode>(ViewMode.Overlay);

/** Which image to show in the primary panel (Original vs Processed) */
export enum ImageDisplayMode {
    Original = "original",
    Processed = "processed",
}
export const imageDisplayMode = writable<ImageDisplayMode>(
    ImageDisplayMode.Processed,
);

/** Overlay opacity (0–1) for overlay mode (applied to image) */
export const overlayOpacity = writable<number>(0.8);

/** Whether an image has been loaded */
export const hasImage = derived(imageBytes, ($bytes) => $bytes !== null);
