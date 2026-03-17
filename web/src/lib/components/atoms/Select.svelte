<script lang="ts">
    interface Option {
        value: string;
        label: string;
    }

    interface Props {
        label: string;
        value: string;
        options: Option[];
        disabled?: boolean;
        compact?: boolean;
        onchange?: (value: string) => void;
    }

    let {
        label,
        value = $bindable(),
        options,
        disabled = false,
        compact = false,
        onchange,
    }: Props = $props();

    function handleChange(e: Event) {
        const target = e.target as HTMLSelectElement;
        value = target.value;
        onchange?.(value);
    }
</script>

<label class="select" class:select--compact={compact}>
    <span class="select__label">{label}</span>

    <div class="select__wrapper">
        <select
            class="select__input"
            {value}
            {disabled}
            onchange={handleChange}
        >
            {#each options as opt}
                <option value={opt.value}>{opt.label}</option>
            {/each}
        </select>

        <span class="select__chevron">▾</span>
    </div>
</label>

<style>
    .select {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
        cursor: pointer;
    }

    .select--compact {
        flex-direction: row;
        align-items: center;
        gap: var(--spacing-sm);
    }

    .select__label {
        font-size: var(--font-sm);
        color: var(--text-secondary);
    }

    .select--compact .select__label {
        font-size: var(--font-xs);
        color: var(--text-secondary);
        flex-shrink: 0;
        white-space: nowrap;
    }

    .select__input {
        width: 100%;
        padding: var(--spacing-sm) var(--spacing-md);
        padding-right: var(--spacing-xl);
        background-color: var(--bg-tertiary);
        border: 1px solid var(--border);
        border-radius: var(--radius-md);
        color: var(--text-primary);
        font-family: inherit;
        font-size: var(--font-sm);
        cursor: pointer;
        appearance: none;
        -webkit-appearance: none;
        transition:
            border-color var(--transition-base),
            background-color var(--transition-base);
    }

    .select__input:hover:not(:disabled) {
        border-color: var(--border-light);
    }

    .select__input:focus {
        outline: none;
        border-color: var(--accent);
    }

    .select__input:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .select__chevron {
        position: absolute;
        right: var(--spacing-sm);
        top: 50%;
        transform: translateY(-50%);
        color: var(--text-muted);
        pointer-events: none;
        font-size: var(--font-sm);
    }

    /* Style the dropdown options */
    .select__input option {
        background-color: var(--bg-secondary);
        color: var(--text-primary);
    }
</style>
