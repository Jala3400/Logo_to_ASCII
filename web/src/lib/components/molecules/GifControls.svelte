<script lang="ts">
    import { asciiGifPlayer } from "$lib/stores";
    import "$lib/web_components/ascii_gif_renderer";
    import Button from "../atoms/Button.svelte";
    import Slider from "../atoms/Slider.svelte";

    let currentFrame = $state(0);
    let isPlaying = $state(false);
    let totalFrames = $state(0);

    function syncFromPlayer() {
        if (!$asciiGifPlayer) return;
        currentFrame = $asciiGifPlayer.currentFrame;
        totalFrames = $asciiGifPlayer.totalFrames;
        isPlaying = $asciiGifPlayer.playing;
    }

    $effect(() => {
        if (!$asciiGifPlayer) return;

        const onFrame = (e: CustomEvent<{ index: number }>) => {
            currentFrame = e.detail.index;
            totalFrames = $asciiGifPlayer?.totalFrames ?? 0;
            isPlaying = $asciiGifPlayer?.playing ?? false;
        };

        $asciiGifPlayer.addEventListener(
            "ascii-frame",
            onFrame as EventListener,
        );

        return () => {
            $asciiGifPlayer?.removeEventListener(
                "ascii-frame",
                onFrame as EventListener,
            );
        };
    });

    function play() {
        $asciiGifPlayer?.play();
        syncFromPlayer();
    }

    function pause() {
        $asciiGifPlayer?.pause();
        syncFromPlayer();
    }

    function seek(i: number) {
        $asciiGifPlayer?.setFrame(i);
        syncFromPlayer();
    }

    function setSpeed(x: number) {
        $asciiGifPlayer?.setSpeed(x);
    }
</script>

<div id="gif-controls">
    <Button onclick={() => (isPlaying ? pause() : play())} size="sm">
        {isPlaying ? "Pause" : "Play"}
    </Button>

    <Slider
        label="Frame"
        max={Math.max(0, totalFrames - 1)}
        value={currentFrame}
        compact={true}
        oninput={(e) => seek(e)}
    />

    <Slider
        label="Speed"
        min={0.1}
        max={4}
        step={0.1}
        value={1}
        compact={true}
        oninput={(speed) => setSpeed(speed)}
    />
</div>

<style>
    #gif-controls {
        width: 100%;
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
    }
</style>
