<script lang="ts">
	import Section from '../atoms/Section.svelte';
	import Toggle from '../atoms/Toggle.svelte';
	import Slider from '../atoms/Slider.svelte';
	import { config } from '$lib/stores';
	import { updateConfig } from '$lib/converter';

	function handleColorChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const hex = target.value;
		const r = parseInt(hex.slice(1, 3), 16);
		const g = parseInt(hex.slice(3, 5), 16);
		const b = parseInt(hex.slice(5, 7), 16);
		updateConfig('transparent_color', [r, g, b]);
	}

	function rgbToHex(rgb: [number, number, number]) {
		const r = rgb[0].toString(16).padStart(2, '0');
		const g = rgb[1].toString(16).padStart(2, '0');
		const b = rgb[2].toString(16).padStart(2, '0');
		return `#${r}${g}${b}`;
	}
</script>

<Section title="Image Processing">
	<div class="color-picker-row">
		<span class="label">Background Color</span>
		<input
			type="color"
			value={rgbToHex($config.transparent_color)}
			oninput={handleColorChange}
			class="color-input"
		/>
	</div>
	<Toggle
		label="Negative"
		checked={$config.negative}
		onchange={(v) => updateConfig('negative', v)}
	/>
	<Toggle
		label="Black & White"
		checked={$config.black_and_white}
		onchange={(v) => updateConfig('black_and_white', v)}
	/>
	<Toggle
		label="Saturate"
		checked={$config.saturate}
		onchange={(v) => updateConfig('saturate', v)}
	/>
	<Toggle
		label="Grayscale"
		checked={$config.grayscale}
		onchange={(v) => updateConfig('grayscale', v)}
	/>
	<Slider
		label="Threshold"
		value={$config.threshold}
		min={0}
		max={1}
		step={0.01}
		onchange={(v) => updateConfig('threshold', v)}
	/>
	<Slider
		label="Midpoint brightness"
		value={$config.midpoint_brightness}
		min={0}
		max={1}
		step={0.01}
		onchange={(v) => updateConfig('midpoint_brightness', v)}
	/>
</Section>

<style>
	.color-picker-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--spacing-sm);
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
