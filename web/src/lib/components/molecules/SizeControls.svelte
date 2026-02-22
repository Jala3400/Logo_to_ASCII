<script lang="ts">
	import Section from '../atoms/Section.svelte';
	import Slider from '../atoms/Slider.svelte';
	import Toggle from '../atoms/Toggle.svelte';
	import { config } from '$lib/stores';
	import { updateConfig } from '$lib/converter';
</script>

<Section title="Size & Position">
	<Slider
		label="Width (chars)"
		value={$config.width_in_chars ?? 0}
		min={0}
		max={200}
		step={1}
		onchange={(v) => updateConfig('width_in_chars', v === 0 ? null : v)}
	/>
	<Slider
		label="Height (chars)"
		value={$config.height_in_chars ?? 0}
		min={0}
		max={200}
		step={1}
		onchange={(v) => updateConfig('height_in_chars', v === 0 ? null : v)}
	/>
	<Slider
		label="Padding"
		value={$config.padding}
		min={0}
		max={20}
		step={1}
		onchange={(v) => updateConfig('padding', v)}
	/>
	<Slider
		label="Padding X"
		value={$config.padding_x}
		min={0}
		max={20}
		step={1}
		onchange={(v) => updateConfig('padding_x', v)}
	/>
	<Slider
		label="Padding Y"
		value={$config.padding_y}
		min={0}
		max={20}
		step={1}
		onchange={(v) => updateConfig('padding_y', v)}
	/>
	<Toggle
		label="Center image"
		checked={$config.center}
		onchange={(v) => updateConfig('center', v)}
	/>
</Section>
