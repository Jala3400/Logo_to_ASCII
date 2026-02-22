<script lang="ts">
	interface Props {
		label: string;
		checked: boolean;
		disabled?: boolean;
		onchange?: (checked: boolean) => void;
	}

	let { label, checked = $bindable(), disabled = false, onchange }: Props = $props();

	function handleChange(e: Event) {
		const target = e.target as HTMLInputElement;
		checked = target.checked;
		onchange?.(checked);
	}
</script>

<label class="toggle" class:toggle--disabled={disabled}>
	<input
		type="checkbox"
		class="toggle__input"
		bind:checked
		{disabled}
		onchange={handleChange}
	/>
	<span class="toggle__switch"></span>
	<span class="toggle__label">{label}</span>
</label>

<style>
	.toggle {
		display: flex;
		align-items: center;
		gap: var(--spacing-sm);
		cursor: pointer;
		user-select: none;
	}

	.toggle--disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.toggle__input {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.toggle__switch {
		position: relative;
		width: 36px;
		height: 20px;
		background-color: var(--bg-tertiary);
		border: 1px solid var(--border);
		border-radius: 10px;
		transition:
			background-color var(--transition-base),
			border-color var(--transition-base);
		flex-shrink: 0;
	}

	.toggle__switch::after {
		content: '';
		position: absolute;
		top: 2px;
		left: 2px;
		width: 14px;
		height: 14px;
		border-radius: 50%;
		background-color: var(--text-muted);
		transition:
			transform var(--transition-base),
			background-color var(--transition-base);
	}

	.toggle__input:checked + .toggle__switch {
		background-color: var(--accent-muted);
		border-color: var(--accent);
	}

	.toggle__input:checked + .toggle__switch::after {
		transform: translateX(16px);
		background-color: var(--accent);
	}

	.toggle__label {
		font-size: var(--font-sm);
		color: var(--text-secondary);
	}
</style>
