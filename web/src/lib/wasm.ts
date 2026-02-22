import init, {
    convert,
    get_final_chars,
    type ConvertResult,
} from "$wasm/logo_to_ascii.js";

let initialized = false;

export async function initWasm(): Promise<void> {
    if (initialized) return;
    await init();
    initialized = true;
}

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
    format: "ansi" | "html";

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
    dicts: ["default"],
    char_size: 16,
    format: "html",
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
    print_color: false,
    border_criteria: null,
    border_thickness: null,
    border_color: [0, 0, 0, 255],
    color_diff: 30.0,
    brightness_diff: 0.1,
    alpha_diff: 0.0,
    algorithm: "max_prod",
};

export interface ConvertOutput {
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
): Promise<ConvertOutput> {
    await initWasm();

    // Build the config object, only including non-null values
    const cfg: Record<string, unknown> = {};
    const merged = { ...DEFAULT_CONFIG, ...config };

    for (const [key, value] of Object.entries(merged)) {
        if (value !== null) {
            cfg[key] = value;
        }
    }

    const result: ConvertResult = convert(imageBytes, cfg);

    const ascii = result.ascii;
    const pngBytes = result.image_png;

    const blob = new Blob([pngBytes as any], { type: "image/png" });
    const imagePngUrl = URL.createObjectURL(blob);

    result.free();

    return { ascii, imagePngUrl };
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
