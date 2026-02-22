<script lang="ts">
	import Section from '../atoms/Section.svelte';
	import Select from '../atoms/Select.svelte';
	import Toggle from '../atoms/Toggle.svelte';
	import { config } from '$lib/stores';
	import { updateConfig } from '$lib/converter';

	const algorithmOptions = [
		{ value: 'max_prod', label: 'Max Product' },
		{ value: 'min_diff', label: 'Min Difference' },
		{ value: 'min_diff_sq', label: 'Min Difference Squared' },
		{ value: 'gradient', label: 'Gradient' },
		{ value: 'corr', label: 'Correlation' },
		{ value: 'ncc', label: 'NCC' }
	];

	const formatOptions = [
		{ value: 'html', label: 'HTML (colored)' },
		{ value: 'ansi', label: 'ANSI (terminal)' }
	];
</script>

<Section title="Algorithm & Output">
	<Select
		label="Algorithm"
		value={$config.algorithm}
		options={algorithmOptions}
		onchange={(v) => updateConfig('algorithm', v as typeof $config.algorithm)}
	/>
	<Select
		label="Output format"
		value={$config.format}
		options={formatOptions}
		onchange={(v) => updateConfig('format', v as typeof $config.format)}
	/>
	<Toggle
		label="Color output"
		checked={$config.print_color}
		onchange={(v) => updateConfig('print_color', v)}
	/>
</Section>
