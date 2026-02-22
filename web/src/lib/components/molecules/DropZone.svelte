<script lang="ts">
	interface Props {
		onfile: (file: File) => void;
		disabled?: boolean;
	}

	let { onfile, disabled = false }: Props = $props();

	let dragover = $state(false);
	let fileInput: HTMLInputElement;

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		dragover = false;
		if (disabled) return;
		const file = e.dataTransfer?.files[0];
		if (file && file.type.startsWith('image/')) {
			onfile(file);
		}
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		if (!disabled) dragover = true;
	}

	function handleDragLeave() {
		dragover = false;
	}

	function handleFileSelect(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			onfile(file);
		}
		// Reset so the same file can be selected again
		target.value = '';
	}

	function openFilePicker() {
		if (!disabled) fileInput.click();
	}
</script>

<div
	class="dropzone"
	class:dropzone--active={dragover}
	class:dropzone--disabled={disabled}
	role="button"
	tabindex="0"
	ondrop={handleDrop}
	ondragover={handleDragOver}
	ondragleave={handleDragLeave}
	onclick={openFilePicker}
	onkeydown={(e) => e.key === 'Enter' && openFilePicker()}
>
	<input
		bind:this={fileInput}
		type="file"
		accept="image/*"
		class="dropzone__input"
		onchange={handleFileSelect}
	/>
	<div class="dropzone__content">
		<span class="dropzone__icon">📁</span>
		<span class="dropzone__text">Drop an image here or click to browse</span>
		<span class="dropzone__hint">PNG, JPEG, GIF, WebP supported</span>
	</div>
</div>

<style>
	.dropzone {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--spacing-xl) var(--spacing-lg);
		border: 2px dashed var(--border);
		border-radius: var(--radius-lg);
		background-color: var(--bg-secondary);
		cursor: pointer;
		transition:
			border-color var(--transition-base),
			background-color var(--transition-base);
	}

	.dropzone:hover:not(.dropzone--disabled) {
		border-color: var(--accent);
		background-color: var(--bg-tertiary);
	}

	.dropzone--active {
		border-color: var(--accent);
		background-color: var(--bg-tertiary);
	}

	.dropzone--disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.dropzone__input {
		display: none;
	}

	.dropzone__content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--spacing-sm);
		text-align: center;
	}

	.dropzone__icon {
		font-size: 2rem;
	}

	.dropzone__text {
		font-size: var(--font-sm);
		color: var(--text-secondary);
	}

	.dropzone__hint {
		font-size: var(--font-xs);
		color: var(--text-muted);
	}
</style>
