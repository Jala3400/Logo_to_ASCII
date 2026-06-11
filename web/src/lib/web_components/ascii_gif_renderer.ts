/**
 * <ascii-player> — Web Component
 *
 * Renders an AsciiAnimation frame-by-frame inside a <pre> element.
 * All visual styling is left to the consumer.
 *
 * API
 * ───
 * Methods
 *   player.load(animation)        Load an AsciiAnimation object. Resets state.
 *   player.play()                 Start playback from the current frame.
 *   player.pause()                Pause playback.
 *   player.setFrame(index)        Jump to a specific frame (0-based, wraps).
 *   player.setSpeed(multiplier)   Set playback speed (1 = normal, 2 = 2×, 0.5 = half).
 *
 * Readonly properties
 *   player.currentFrame           Current frame index (0-based).
 *   player.totalFrames            Total number of frames, or 0 if nothing loaded.
 *   player.playing                true while playing.
 *
 * Events (dispatched on the element)
 *   ascii-frame   CustomEvent<AsciiFrameEvent>   Fired on every frame advance.
 *   ascii-end     CustomEvent<Record<never,never>>   Fired when the last frame plays.
 *
 * Attributes (optional)
 *   autoplay      If present, starts playing automatically after load().
 *
 * Styling
 *   Style the inner <pre> from outside using ::part(pre):
 *     ascii-player::part(pre) { color: lime; font-size: 14px; }
 */

export interface AsciiFrame {
    ascii: string;
    delay_ms: number;
}

export interface AsciiAnimation {
    font: string;
    width: number;
    height: number;
    frames: AsciiFrame[];
}

export interface AsciiFrameEvent {
    index: number;
    frame: AsciiFrame;
}

// Augment the global HTMLElementEventMap so addEventListener is fully typed
// when working with a reference typed as AsciiPlayerElement.
declare global {
    interface AsciiPlayerElementEventMap {
        "ascii-frame": CustomEvent<AsciiFrameEvent>;
        "ascii-end": CustomEvent<Record<never, never>>;
    }

    interface HTMLElementTagNameMap {
        "ascii-player": AsciiPlayerElement;
    }
}

export interface AsciiPlayerElement extends HTMLElement {
    load(animation: AsciiAnimation): void;
    play(): void;
    pause(): void;
    setFrame(index: number): void;
    setSpeed(multiplier: number): void;
    readonly currentFrame: number;
    readonly totalFrames: number;
    readonly playing: boolean;
    addEventListener<K extends keyof AsciiPlayerElementEventMap>(
        type: K,
        listener: (
            this: AsciiPlayerElement,
            ev: AsciiPlayerElementEventMap[K],
        ) => void,
        options?: boolean | AddEventListenerOptions,
    ): void;
    removeEventListener<K extends keyof AsciiPlayerElementEventMap>(
        type: K,
        listener: (
            this: AsciiPlayerElement,
            ev: AsciiPlayerElementEventMap[K],
        ) => void,
        options?: boolean | EventListenerOptions,
    ): void;
}

// ── Implementation ────────────────────────────────────────────────────────────

class AsciiPlayer extends HTMLElement implements AsciiPlayerElement {
    // ── private state ───────────────────────────────────────────────────────────
    #anim: AsciiAnimation | null = null;
    #idx: number = 0;
    #speed: number = 1;
    #timer: ReturnType<typeof setTimeout> | null = null;
    #playing: boolean = false;
    #pre: HTMLPreElement;

    // ── shadow DOM setup ────────────────────────────────────────────────────────
    constructor() {
        super();
        const root = this.attachShadow({ mode: "open" });
        root.innerHTML = `
      <style>
        :host { display: block; }
        pre {
          margin: 0;
          padding: 0;
          white-space: pre;
          font-family: inherit;
          font-size: inherit;
          line-height: inherit;
          background: none;
          border: none;
        }
      </style>
      <pre part="pre"></pre>
    `;
        this.#pre = root.querySelector("pre") as HTMLPreElement;
    }

    // ── public API ──────────────────────────────────────────────────────────────

    /** Load an AsciiAnimation. Stops any active playback and resets to frame 0. */
    load(animation: AsciiAnimation): void {
        this.#validate(animation);
        this.#stop();
        this.#anim = animation;
        this.#idx = 0;
        this.#speed = 1;
        this.#pre.style.fontFamily = `"${animation.font}", monospace`;
        this.#render();
        if (this.hasAttribute("autoplay")) this.play();
    }

    /** Start playback from the current frame. No-op if already playing or nothing loaded. */
    play(): void {
        if (this.#playing || !this.#anim) return;
        this.#playing = true;
        this.#schedule();
    }

    /** Pause playback. No-op if already paused. */
    pause(): void {
        this.#stop();
    }

    /**
     * Jump to a specific frame (0-based).
     * Wraps around if index is out of range.
     * Does NOT start or stop playback.
     */
    setFrame(index: number): void {
        if (!this.#anim) return;
        this.#idx = this.#wrap(index);
        this.#render();
    }

    /**
     * Set playback speed multiplier.
     * 1 = normal, 2 = twice as fast, 0.5 = half speed.
     * Takes effect on the next frame transition.
     */
    setSpeed(multiplier: number): void {
        if (typeof multiplier !== "number" || multiplier <= 0) {
            throw new TypeError("speed must be a positive number");
        }
        this.#speed = multiplier;
    }

    // ── readonly properties ─────────────────────────────────────────────────────

    get currentFrame(): number {
        return this.#idx;
    }
    get totalFrames(): number {
        return this.#anim?.frames.length ?? 0;
    }
    get playing(): boolean {
        return this.#playing;
    }

    // ── private helpers ─────────────────────────────────────────────────────────

    #render(): void {
        if (!this.#anim) return;
        const frame = this.#anim.frames[this.#idx];
        this.#pre.innerHTML = frame.ascii;

        this.dispatchEvent(
            new CustomEvent<AsciiFrameEvent>("ascii-frame", {
                bubbles: true,
                detail: { index: this.#idx, frame },
            }),
        );

        if (this.#idx === this.#anim.frames.length - 1) {
            this.dispatchEvent(
                new CustomEvent("ascii-end", { bubbles: true, detail: {} }),
            );
        }
    }

    #schedule(): void {
        const delay = this.#anim!.frames[this.#idx].delay_ms / this.#speed;
        this.#timer = setTimeout(() => {
            this.#idx = this.#wrap(this.#idx + 1);
            this.#render();
            if (this.#playing) this.#schedule();
        }, delay);
    }

    #stop(): void {
        if (this.#timer !== null) clearTimeout(this.#timer);
        this.#timer = null;
        this.#playing = false;
    }

    #wrap(index: number): number {
        const n = this.#anim!.frames.length;
        return ((index % n) + n) % n;
    }

    #validate(d: unknown): asserts d is AsciiAnimation {
        if (!d || typeof d !== "object")
            throw new TypeError("animation must be an object");
        const a = d as Record<string, unknown>;
        if (typeof a["font"] !== "string")
            throw new TypeError("animation.font must be a string");
        if (typeof a["width"] !== "number")
            throw new TypeError("animation.width must be a number");
        if (typeof a["height"] !== "number")
            throw new TypeError("animation.height must be a number");
        if (!Array.isArray(a["frames"]) || !(a["frames"] as unknown[]).length)
            throw new TypeError("animation.frames must be a non-empty array");
        for (let i = 0; i < (a["frames"] as unknown[]).length; i++) {
            const f = (a["frames"] as Record<string, unknown>[])[i];
            if (typeof f["ascii"] !== "string")
                throw new TypeError(`frames[${i}].ascii must be a string`);
            if (typeof f["delay_ms"] !== "number")
                throw new TypeError(`frames[${i}].delay_ms must be a number`);
        }
    }

    // ── lifecycle ───────────────────────────────────────────────────────────────

    disconnectedCallback(): void {
        this.#stop();
    }
}

customElements.define("ascii-player", AsciiPlayer);
