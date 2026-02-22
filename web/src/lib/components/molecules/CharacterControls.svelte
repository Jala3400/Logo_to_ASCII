<script lang="ts">
    import { updateConfig } from "$lib/converter";
    import { config } from "$lib/stores";
    import { getFinalChars } from "$lib/wasm";
    import Section from "../atoms/Section.svelte";
    import Select from "../atoms/Select.svelte";
    import TextInput from "../atoms/TextInput.svelte";

    const charSizeOptions = [
        { value: "8", label: "8px" },
        { value: "12", label: "12px" },
        { value: "16", label: "16px (default)" },
        { value: "20", label: "20px" },
        { value: "24", label: "24px" },
        { value: "32", label: "32px" },
    ];

    let charSizeStr = $derived(String($config.char_size));

    const dictOptions = [
        { value: "default", label: "Default" },
        { value: "all", label: "All ASCII" },
        { value: "symbols", label: "Symbols" },
        { value: "blocks", label: "Blocks" },
        { value: "blocks_all", label: "Blocks (Extended)" },
        { value: "box", label: "Box Drawing" },
        { value: "box_all", label: "Box Drawing (Extended)" },
        { value: "box_double", label: "Box Drawing (Double)" },
        { value: "box_double_all", label: "Box Drawing (Double, Extended)" },
        { value: "nerd", label: "Nerd Font Symbols" },
        { value: "math", label: "Mathematical Symbols" },
        { value: "numbers", label: "Numbers" },
        { value: "letters", label: "Letters" },
    ];

    let selectedDict = $derived($config.dicts?.[0] ?? "custom");
    let finalChars = $state("");

    function handleDictChange(v: string) {
        if (v === "custom") {
            updateConfig("dicts", null);
        } else {
            updateConfig("dicts", [v]);
        }
    }

    $effect(() => {
        // Reactively update the character preview using the WASM logic
        const trigger = {
            c: $config.chars,
            a: $config.add_chars,
            e: $config.except,
            d: $config.dicts,
        };
        getFinalChars($config).then((c) => (finalChars = c));
    });
</script>

<Section title="Characters">
    <Select
        label="Character set preset"
        value={selectedDict}
        options={dictOptions}
        onchange={handleDictChange}
    />

    {#if selectedDict === "custom"}
        <TextInput
            label="Base Characters"
            value={$config.chars}
            placeholder="8dbqp'·. "
            onchange={(v) => updateConfig("chars", v)}
        />
    {/if}

    <TextInput
        label="Add characters"
        value={$config.add_chars}
        placeholder="Extra characters..."
        onchange={(v) => updateConfig("add_chars", v)}
    />

    <TextInput
        label="Exclude characters"
        value={$config.except}
        placeholder="Characters to exclude..."
        onchange={(v) => updateConfig("except", v)}
    />

    <div class="preview">
        <span class="preview__label">Current Character Set:</span>
        <div class="preview__box">
            {finalChars}
        </div>
        <div class="preview__count">
            {finalChars.length} characters total
        </div>
    </div>

    <Select
        label="Character size"
        value={charSizeStr}
        options={charSizeOptions}
        onchange={(v) => updateConfig("char_size", parseInt(v))}
    />
</Section>

<style>
    .preview {
        margin-top: var(--spacing-sm);
        margin-bottom: var(--spacing-md);
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
    }

    .preview__label {
        font-size: var(--font-size-xs);
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .preview__box {
        padding: var(--spacing-sm);
        background: var(--bg-primary);
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        font-family: var(--font-mono);
        font-size: var(--font-size-md);
        word-break: break-all;
        min-height: 2.5rem;
        line-height: 1.4;
        color: var(--accent);
    }

    .preview__count {
        font-size: var(--font-size-xs);
        color: var(--text-muted);
        text-align: right;
    }
</style>
