<script lang="ts">
    interface Props {
        label: string;
        value: string;
        placeholder?: string;
        disabled?: boolean;
        onchange?: (value: string) => void;
        onkeydown?: (e: KeyboardEvent) => void;
    }

    let {
        label,
        value = $bindable(),
        placeholder = "",
        disabled = false,
        onchange,
        onkeydown,
    }: Props = $props();

    function handleInput(e: Event) {
        const target = e.target as HTMLInputElement;
        value = target.value;
        onchange?.(value);
    }
</script>

<label class="text-input">
    <span class="text-input__label">{label}</span>

    <input
        type="text"
        class="text-input__input"
        {value}
        {placeholder}
        {disabled}
        oninput={handleInput}
        {onkeydown}
    />
</label>

<style>
    .text-input {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
        cursor: text;
    }

    .text-input__label {
        font-size: var(--font-sm);
        color: var(--text-secondary);
    }

    .text-input__input {
        width: 100%;
        padding: var(--spacing-sm) var(--spacing-md);
        background-color: var(--bg-tertiary);
        border: 1px solid var(--border);
        border-radius: var(--radius-md);
        color: var(--text-primary);
        font-family: inherit;
        font-size: var(--font-sm);
        transition: border-color var(--transition-base);
    }

    .text-input__input:hover:not(:disabled) {
        border-color: var(--border-light);
    }

    .text-input__input:focus {
        outline: none;
        border-color: var(--accent);
    }

    .text-input__input:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .text-input__input::placeholder {
        color: var(--text-muted);
    }
</style>
