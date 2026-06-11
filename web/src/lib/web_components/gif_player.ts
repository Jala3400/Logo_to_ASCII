/**
 * <gif-player> — Web Component
 *
 * Renders a GifFrameOutput[] sequence frame-by-frame inside an <img> element.
 * Mirrors the ascii-player API exactly so both can be driven by the same controller.
 *
 * API
 * ───
 * Methods
 *   player.load(frames)           Load a GifFrameOutput[]. Resets state.
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
 *   gif-frame   CustomEvent<GifFrameEvent>           Fired on every frame advance.
 *   gif-end     CustomEvent<Record<never, never>>    Fired when the last frame plays.
 *
 * Attributes (optional)
 *   autoplay    If present, starts playing automatically after load().
 *
 * Styling
 *   Style the inner <img> from outside using ::part(img):
 *     gif-player::part(img) { width: 320px; image-rendering: pixelated; }
 */

export interface GifFrameOutput {
  delayMs: number
  pngUrl: string
}

export interface GifFrameEvent {
  index: number
  frame: GifFrameOutput
}

declare global {
  interface GifPlayerElementEventMap {
    'gif-frame': CustomEvent<GifFrameEvent>
    'gif-end':   CustomEvent<Record<never, never>>
  }

  interface HTMLElementTagNameMap {
    'gif-player': GifPlayerElement
  }
}

export interface GifPlayerElement extends HTMLElement {
  load(frames: GifFrameOutput[]): void
  play(): void
  pause(): void
  setFrame(index: number): void
  setSpeed(multiplier: number): void
  readonly currentFrame: number
  readonly totalFrames: number
  readonly playing: boolean
  addEventListener<K extends keyof GifPlayerElementEventMap>(
    type: K,
    listener: (this: GifPlayerElement, ev: GifPlayerElementEventMap[K]) => void,
    options?: boolean | AddEventListenerOptions
  ): void
  removeEventListener<K extends keyof GifPlayerElementEventMap>(
    type: K,
    listener: (this: GifPlayerElement, ev: GifPlayerElementEventMap[K]) => void,
    options?: boolean | EventListenerOptions
  ): void
}

// ── Implementation ────────────────────────────────────────────────────────────

class GifPlayer extends HTMLElement implements GifPlayerElement {
  #frames:  GifFrameOutput[] = []
  #idx:     number  = 0
  #speed:   number  = 1
  #timer:   ReturnType<typeof setTimeout> | null = null
  #playing: boolean = false
  #img:     HTMLImageElement

  constructor() {
    super()
    const root = this.attachShadow({ mode: 'open' })
    root.innerHTML = `
      <style>
        :host { display: block; }
        img {
          display: block;
          height: auto;
        }
      </style>
      <img part="img" alt="" />
    `
    this.#img = root.querySelector('img') as HTMLImageElement
  }

  // ── public API ──────────────────────────────────────────────────────────────

  load(frames: GifFrameOutput[]): void {
    if (!Array.isArray(frames) || frames.length === 0)
      throw new TypeError('frames must be a non-empty array')
    this.#stop()
    this.#frames = frames
    this.#idx    = 0
    this.#speed  = 1
    this.#render()
    if (this.hasAttribute('autoplay')) this.play()
  }

  play(): void {
    if (this.#playing || !this.#frames.length) return
    this.#playing = true
    this.#schedule()
  }

  pause(): void {
    this.#stop()
  }

  setFrame(index: number): void {
    if (!this.#frames.length) return
    this.#idx = this.#wrap(index)
    this.#render()
  }

  setSpeed(multiplier: number): void {
    if (typeof multiplier !== 'number' || multiplier <= 0)
      throw new TypeError('speed must be a positive number')
    this.#speed = multiplier
  }

  // ── readonly properties ─────────────────────────────────────────────────────

  get currentFrame(): number  { return this.#idx }
  get totalFrames():  number  { return this.#frames.length }
  get playing():      boolean { return this.#playing }

  // ── private helpers ─────────────────────────────────────────────────────────

  #render(): void {
    const frame = this.#frames[this.#idx]
    this.#img.src = frame.pngUrl
    this.dispatchEvent(new CustomEvent<GifFrameEvent>('gif-frame', {
      bubbles: true,
      detail: { index: this.#idx, frame }
    }))
    if (this.#idx === this.#frames.length - 1) {
      this.dispatchEvent(new CustomEvent('gif-end', { bubbles: true, detail: {} }))
    }
  }

  #schedule(): void {
    const delay = this.#frames[this.#idx].delayMs / this.#speed
    this.#timer = setTimeout(() => {
      this.#idx = this.#wrap(this.#idx + 1)
      this.#render()
      if (this.#playing) this.#schedule()
    }, Math.max(16, delay))
  }

  #stop(): void {
    if (this.#timer !== null) clearTimeout(this.#timer)
    this.#timer   = null
    this.#playing = false
  }

  #wrap(index: number): number {
    const n = this.#frames.length
    return ((index % n) + n) % n
  }

  disconnectedCallback(): void {
    this.#stop()
  }
}

customElements.define('gif-player', GifPlayer)