import { derived, writable } from "svelte/store";
import { DEFAULT_CONFIG, type L2aConfig } from "./wasm";
import type {
    AsciiAnimation,
    AsciiPlayerElement,
} from "./web_components/ascii_gif_renderer";
import type { GifFrameOutput } from "./web_components/gif_player";

/** User configuration (reactive — triggers re-conversion) */
export const config = writable<L2aConfig>({ ...DEFAULT_CONFIG });

/** ========== Image data ========== */

/** Currently loaded image as raw bytes */
export const imageBytes = writable<Uint8Array | null>(null);

/** Original image object URL for preview */
export const originalImageUrl = writable<string | null>(null);

/** Processed image blob URL */
export const processedImageUrl = writable<string | null>(null);

/** ASCII output */
export const asciiImageOutput = writable<string | null>(null);

/** ========== Gif data ========== */

/** Currently loaded GIF as raw bytes */
export const gifBytes = writable<Uint8Array | null>(null);

/** Original GIF object URL for preview */
export const originalGif = writable<GifFrameOutput[] | null>(null);

/** Processed GIF blob URL */
export const processedGif = writable<GifFrameOutput[] | null>(null);

/** ASCII output for GIFs */
export const asciiGifOutput = writable<AsciiAnimation | null>(null);

/** GIF player */
export const asciiGifPlayer = writable<AsciiPlayerElement | null>(null);

/** ========== Status flags ========== */

/** Whether a conversion is currently running */
export const isConverting = writable<boolean>(false);

/** Error message from last conversion, if any */
export const errorMessage = writable<string | null>(null);

/** Whether the WASM module has been initialized */
export const wasmReady = writable<boolean>(false);

/** ========== UI state ========== */

/** Current type of file */
export enum FileType {
    Image = "image",
    Gif = "gif",
    None = "none",
}
export const fileType = writable<FileType>(FileType.None);

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
export const overlayOpacity = writable<number>(0);

/** Whether the sidebar is collapsed */
export const sidebarCollapsed = writable<boolean>(false);

/** Whether an image has been loaded */
export const showResult = derived(fileType, ($type) => $type !== FileType.None);
