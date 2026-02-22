<script lang="ts">
    import { onMount } from "svelte";
    import Header from "$lib/components/organisms/Header.svelte";
    import Sidebar from "$lib/components/organisms/Sidebar.svelte";
    import PreviewArea from "$lib/components/organisms/PreviewArea.svelte";
    import Button from "$lib/components/atoms/Button.svelte";
    import { initialize } from "$lib/converter";

    onMount(() => {
        initialize();
    });
</script>

<svelte:head>
    <title>Logo to ASCII</title>
    <meta
        name="description"
        content="Convert images to ASCII art in your browser"
    />
</svelte:head>

<div class="app-layout">
    <Header>
        <Button
            size="sm"
            variant="ghost"
            onclick={() =>
                window.open(
                    "https://github.com/Jala3400/Logo_to_ASCII",
                    "_blank",
                )}
        >
            GitHub
        </Button>
    </Header>
    <div class="app-layout__body">
        <Sidebar />
        <PreviewArea />
    </div>
</div>

<style>
    .app-layout {
        display: flex;
        flex-direction: column;
        height: 100vh;
        overflow: hidden;
    }

    .app-layout__body {
        display: flex;
        flex: 1;
        overflow: hidden;
    }
</style>
