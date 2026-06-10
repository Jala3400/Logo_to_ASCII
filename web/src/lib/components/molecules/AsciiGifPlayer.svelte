<script lang="ts">
    import { asciiGifPlayer } from "$lib/stores";
    import "$lib/web_components/ascii_gif_renderer";
    import type { AsciiAnimation } from "$lib/web_components/ascii_gif_renderer";

    let { animation, autoplay = true } = $props<{
        animation: AsciiAnimation;
        autoplay?: boolean;
    }>();

    $effect(() => {
        if (!$asciiGifPlayer || !animation) return;
        $asciiGifPlayer.load(animation);
        if (autoplay) $asciiGifPlayer.play();
    });
</script>

<ascii-player bind:this={$asciiGifPlayer} {autoplay}></ascii-player>
