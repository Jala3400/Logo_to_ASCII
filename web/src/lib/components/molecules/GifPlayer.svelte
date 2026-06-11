<script lang="ts">
    import { asciiGifPlayer } from "$lib/stores";
    import "$lib/web_components/gif_player";
    import type {
        GifFrameOutput,
        GifPlayerElement,
    } from "$lib/web_components/gif_player";

    let { animation } = $props<{
        animation: GifFrameOutput[];
    }>();

    let player: GifPlayerElement;

    $effect(() => {
        if (!$asciiGifPlayer || !player) return;

        const onFrame = (e: CustomEvent<{ index: number }>) => {
            player.setFrame(e.detail.index);
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

    $effect(() => {
        if (!player || !animation) return;
        player.load(animation);
        player.setFrame($asciiGifPlayer?.currentFrame ?? 0);
    });
</script>

<gif-player bind:this={player}></gif-player>
