import type { GifFrameOutput } from "$lib/wasm";
import { applyPalette, GIFEncoder, quantize } from "gifenc";

export async function makeGif(frames: GifFrameOutput[]): Promise<Blob> {
    const encoder = GIFEncoder();

    for (const frame of frames) {
        const { data, width, height } = await pngUrlToImageData(frame.pngUrl);
        const palette = quantize(data, 256);
        const index = applyPalette(data, palette);
        encoder.writeFrame(index, width, height, {
            palette,
            delay: frame.delayMs, // gifenc expects ms, same unit as your type
        });
    }

    encoder.finish();
    return new Blob([encoder.bytes()], { type: "image/gif" });
}

function pngUrlToImageData(
    url: string,
): Promise<{ data: Uint8ClampedArray; width: number; height: number }> {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.onload = () => {
            const canvas = document.createElement("canvas");
            canvas.width = img.width;
            canvas.height = img.height;
            const ctx = canvas.getContext("2d");
            if (!ctx) return reject(new Error("Could not get 2D context"));
            ctx.drawImage(img, 0, 0);
            const { data } = ctx.getImageData(0, 0, img.width, img.height);
            resolve({ data, width: img.width, height: img.height });
        };
        img.onerror = () => reject(new Error(`Failed to load frame: ${url}`));
        img.src = url;
    });
}
