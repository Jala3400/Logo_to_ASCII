<script lang="ts">
	import Section from '../atoms/Section.svelte';
	import Toggle from '../atoms/Toggle.svelte';
	import Slider from '../atoms/Slider.svelte';
	import { config } from '$lib/stores';
	import { updateConfig } from '$lib/converter';

	function toggleCriteria(criterion: string) {
		const current = $config.border_criteria ?? [];
		if (current.includes(criterion)) {
			const next = current.filter((c) => c !== criterion);
			updateConfig('border_criteria', next.length > 0 ? next : null);
		} else {
			updateConfig('border_criteria', [...current, criterion]);
		}
	}

	function isChecked(criterion: string) {
		const criteria = $config.border_criteria ?? [];
		if (criterion !== 'all' && criteria.includes('all')) return true;
		return criteria.includes(criterion);
	}

	function handleColorChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const hex = target.value; // #RRGGBB
		const r = parseInt(hex.slice(1, 3), 16);
		const g = parseInt(hex.slice(3, 5), 16);
		const b = parseInt(hex.slice(5, 7), 16);
		updateConfig('border_color', [r, g, b, 255]);
	}

	function rgbToHex(rgba: [number, number, number, number]) {
		const r = rgba[0].toString(16).padStart(2, '0');
		const g = rgba[1].toString(16).padStart(2, '0');
		const b = rgba[2].toString(16).padStart(2, '0');
		return `#${r}${g}${b}`;
	}
</script>

<Section title="Borders">
	<div class="criteria-grid">
		<Toggle
			label="Detect All"
			checked={isChecked('all')}
			onchange={() => toggleCriteria('all')}
		/>
		<Toggle
			label="Color Diff"
			checked={isChecked('color')}
			onchange={() => toggleCriteria('color')}
		/>
		<Toggle
			label="Brightness Diff"
			checked={isChecked('brightness')}
			onchange={() => toggleCriteria('brightness')}
		/>
		<Toggle
			label="Alpha Diff"
			checked={isChecked('alpha')}
			onchange={() => toggleCriteria('alpha')}
		/>
	</div>

	{#if $config.border_criteria}
		<div class="border-settings">
			<div class="color-picker-row">
				<span class="label">Border Color</span>
				<input
					type="color"
					value={rgbToHex($config.border_color)}
					oninput={handleColorChange}
					class="color-input"
				/>
			</div>

			<Slider
				label="Thickness (px)"
				value={$config.border_thickness ?? 1}
				min={1}
				max={20}
				step={1}
				onchange={(v) => updateConfig('border_thickness', v)}
			/>

			{#if isChecked('color')}
				<Slider
					label="Color threshold"
					value={$config.color_diff}
					min={0}
					max={360}
					step={1}
					onchange={(v) => updateConfig('color_diff', v)}
				/>
			{/if}

			{#if isChecked('brightness')}
				<Slider
					label="Brightness threshold"
					value={$config.brightness_diff}
					min={0}
					max={1}
					step={0.01}
					onchange={(v) => updateConfig('brightness_diff', v)}
				/>
			{/if}

			{#if isChecked('alpha')}
				<Slider
					label="Alpha threshold"
					value={$config.alpha_diff}
					min={0}
					max={1}
					step={0.01}
					onchange={(v) => updateConfig('alpha_diff', v)}
				/>
			{/if}
		</div>
	{/if}
</Section>

<style>
	.criteria-grid {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
		margin-bottom: var(--spacing-sm);
	}

	.border-settings {
		margin-top: var(--spacing-sm);
		padding-top: var(--spacing-sm);
		border-top: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: var(--spacing-sm);
	}

	.color-picker-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.label {
		font-size: var(--font-size-sm);
		color: var(--text-secondary);
	}

	.color-input {
		appearance: none;
		-webkit-appearance: none;
		border: 1px solid var(--border);
		border-radius: var(--radius-sm);
		width: 40px;
		height: 24px;
		cursor: pointer;
		background: none;
		padding: 0;
	}

	.color-input::-webkit-color-swatch-wrapper {
		padding: 0;
	}

	.color-input::-webkit-color-swatch {
		border: none;
		border-radius: var(--radius-xs);
	}
</style>
