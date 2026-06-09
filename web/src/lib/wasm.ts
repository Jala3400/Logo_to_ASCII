import init, {
    convert_gif,
    convert_image,
    ConvertGifResult,
    get_final_chars,
    GifFrameInfo,
    type ConvertImageResult,
} from "$wasm/logo_to_ascii.js";
import type { AsciiAnimation } from "./web_components/ascii_gif_renderer";

let initialized = false;

export async function initWasm(): Promise<void> {
    if (initialized) return;
    await init();
    initialized = true;
}

export const OutputFormat = {
    Ansi: "ansi",
    Html: "html",
} as const;

export type OutputFormat = (typeof OutputFormat)[keyof typeof OutputFormat];

export interface L2aConfig {
    // Font
    font_name: string | null;

    // Character set
    chars: string;
    add_chars: string;
    except: string;
    dicts: string[] | null;
    char_size: number;

    // Output
    format: OutputFormat;

    // Image processing
    negative: boolean;
    black_and_white: boolean;
    threshold: number;
    transparent_color: [number, number, number];
    saturate: boolean;
    grayscale: boolean;
    midpoint_brightness: number;

    // Dimensions
    width_in_chars: number | null;
    height_in_chars: number | null;

    // Padding
    padding: number;
    padding_x: number;
    padding_y: number;
    center: boolean;

    // Borders and colors
    print_color: boolean;
    border_criteria: string[] | null;
    border_thickness: number | null;
    border_color: [number, number, number, number];
    color_diff: number;
    brightness_diff: number;
    alpha_diff: number;

    // Algorithm
    algorithm:
        | "max_prod"
        | "min_diff"
        | "min_diff_sq"
        | "gradient"
        | "corr"
        | "ncc";
}

export const DEFAULT_CONFIG: L2aConfig = {
    font_name: "Ubuntu Mono",
    chars: "8dbqp'·. ",
    add_chars: "",
    except: "",
    dicts: ["all"],
    char_size: 16,
    format: OutputFormat.Html,
    negative: false,
    black_and_white: false,
    threshold: 0.5,
    transparent_color: [0, 0, 0],
    saturate: false,
    grayscale: false,
    midpoint_brightness: 0.5,
    width_in_chars: null,
    height_in_chars: null,
    padding: 0,
    padding_x: 0,
    padding_y: 0,
    center: false,
    print_color: true,
    border_criteria: null,
    border_thickness: null,
    border_color: [0, 0, 0, 255],
    color_diff: 30.0,
    brightness_diff: 0.1,
    alpha_diff: 0.0,
    algorithm: "max_prod",
};

export interface ConvertImageOutput {
    ascii: string;
    imagePngUrl: string;
}

/**
 * Convert an image to ASCII art using the WASM module.
 * Returns the ASCII string and a blob URL for the processed image.
 */
export async function convertImage(
    imageBytes: Uint8Array,
    config: Partial<L2aConfig>,
): Promise<ConvertImageOutput> {
    await initWasm();

    // Build the config object, only including non-null values
    const cfg: Record<string, unknown> = {};
    const merged = { ...DEFAULT_CONFIG, ...config };

    for (const [key, value] of Object.entries(merged)) {
        if (value !== null) {
            cfg[key] = value;
        }
    }

    const result: ConvertImageResult = convert_image(imageBytes, cfg);

    const ascii = result.ascii;
    const pngBytes = result.image_png;

    const blob = new Blob([pngBytes as any], { type: "image/png" });
    const imagePngUrl = URL.createObjectURL(blob);

    result.free();

    return { ascii, imagePngUrl };
}

export interface GifFrameOutput {
    delayMs: number;
    pngUrl: string;
}
export interface ConvertGifOutput {
    ascii_json: AsciiAnimation;
    originalGif: GifFrameOutput[];
    processedGif: GifFrameOutput[];
}

/**
 * Convert a GIF to ASCII art using the WASM module.
 * Returns the ASCII animation and blob URLs for the original and processed GIF frames.
 */
export async function convertGif(
    gifBytes: Uint8Array,
    config: Partial<L2aConfig>,
): Promise<ConvertGifOutput> {
    await initWasm();

    const cfg: Record<string, unknown> = {};
    const merged = { ...DEFAULT_CONFIG, ...config };

    for (const [key, value] of Object.entries(merged)) {
        if (value !== null) cfg[key] = value;
    }

    const result: ConvertGifResult = convert_gif(gifBytes, cfg);

    const toFrameOutput = (frame: GifFrameInfo): GifFrameOutput => {
        const blob = new Blob([frame.png_bytes as any], { type: "image/png" });
        return {
            delayMs: Number(frame.delay_ms),
            pngUrl: URL.createObjectURL(blob),
        };
    };

    const ascii = JSON.parse(result.ascii_json) as AsciiAnimation;

    const output = {
        ascii_json: ascii,
        originalGif: result.original_gif.map(toFrameOutput),
        processedGif: result.processed_gif.map(toFrameOutput),
    };

    result.free();
    return output;
}

// ── Font helpers ──────────────────────────────────────────────────────────────

/**
 * Returns the final character set after running process_characters on `config`.
 */
export async function getFinalChars(
    config: Partial<L2aConfig>,
): Promise<string> {
    await initWasm();

    const cfg: Record<string, unknown> = {};
    const merged = { ...DEFAULT_CONFIG, ...config };
    for (const [key, value] of Object.entries(merged)) {
        if (value !== null) cfg[key] = value;
    }

    return get_final_chars(cfg) as string;
}
