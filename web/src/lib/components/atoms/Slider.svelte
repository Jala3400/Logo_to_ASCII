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
        const val = parseFloat(target.value);
        if (!isNaN(val)) {
            value = val;
            onchange?.(value);
        }
    }

    function handleWheel(e: WheelEvent) {
        if (disabled) return;

        // Determine if we are at the boundaries to allow page scroll if not adjusting
        const isAtMax = value >= max && e.deltaY < 0;
        const isAtMin = value <= min && e.deltaY > 0;

        if (!isAtMax && !isAtMin) {
            e.preventDefault();
            const direction = e.deltaY > 0 ? -1 : 1;
            const newValue = Math.min(
                max,
                Math.max(min, value + direction * step),
            );

            // Correct floating point precision issues
            const decimals = step.toString().split(".")[1]?.length || 0;
            value = parseFloat(newValue.toFixed(decimals));
            onchange?.(value);
        }
    }
</script>

<label class="slider" class:slider--disabled={disabled} onwheel={handleWheel}>
    <div class="slider__header">
        <span class="slider__label">{label}</span>
        <input
            type="number"
            class="slider__num-input"
            {min}
            {max}
            {step}
            {value}
            {disabled}
            oninput={handleInput}
        />
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

    .slider--disabled {
        opacity: 0.5;
        cursor: not-allowed;
        pointer-events: none;
    }

    .slider__header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: var(--spacing-sm);
    }

    .slider__label {
        font-size: var(--font-sm);
        color: var(--text-secondary);
        flex: 1;
    }

    .slider__num-input {
        width: 6ch;
        padding: 2px 4px;
        font-size: var(--font-xs);
        font-family: inherit;
        color: var(--text-primary);
        background: var(--bg-tertiary);
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        text-align: right;
        font-variant-numeric: tabular-nums;
        outline: none;
        transition: border-color var(--transition-base);
    }

    .slider__num-input:focus {
        border-color: var(--accent);
    }

    /* Hide arrows for number input if needed */
    .slider__num-input::-webkit-inner-spin-button,
    .slider__num-input::-webkit-outer-spin-button {
        -webkit-appearance: none;
        margin: 0;
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
        cursor: not-allowed;
    }
</style>
