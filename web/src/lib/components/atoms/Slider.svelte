<script lang="ts">
    interface Props {
        label: string;
        value: number;
        min?: number;
        max?: number;
        step?: number;
        disabled?: boolean;
        onchange?: (value: number) => void;
    }

    let {
        label,
        value = $bindable(),
        min = 0,
        max = 100,
        step = 1,
        disabled = false,
        onchange,
    }: Props = $props();

    function handleInput(e: Event) {
        const target = e.target as HTMLInputElement;
        value = parseFloat(target.value);
        onchange?.(value);
    }
</script>

<label class="slider">
    <div class="slider__header">
        <span class="slider__label">{label}</span>
        <span class="slider__value">{value}</span>
    </div>

    <input
        type="range"
        class="slider__input"
        {min}
        {max}
        {step}
        {value}
        {disabled}
        oninput={handleInput}
    />
</label>

<style>
    .slider {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
        cursor: pointer;
    }

    .slider__header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .slider__label {
        font-size: var(--font-sm);
        color: var(--text-secondary);
    }

    .slider__value {
        font-size: var(--font-xs);
        color: var(--text-muted);
        font-variant-numeric: tabular-nums;
        min-width: 3ch;
        text-align: right;
    }

    .slider__input {
        -webkit-appearance: none;
        appearance: none;
        width: 100%;
        height: 4px;
        border-radius: 2px;
        background: var(--bg-tertiary);
        outline: none;
        cursor: pointer;
    }

    .slider__input::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 14px;
        height: 14px;
        border-radius: 50%;
        background: var(--accent);
        cursor: pointer;
        transition: background-color var(--transition-base);
    }

    .slider__input::-webkit-slider-thumb:hover {
        background: var(--accent-hover);
    }

    .slider__input::-moz-range-thumb {
        width: 14px;
        height: 14px;
        border: none;
        border-radius: 50%;
        background: var(--accent);
        cursor: pointer;
    }

    .slider__input:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
</style>
