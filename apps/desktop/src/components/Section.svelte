<script lang="ts">
	import { Spacer } from '@gitbutler/ui';
	import { pxToRem } from '@gitbutler/ui/utils/pxToRem';
	import type { Snippet } from 'svelte';

	interface Props {
		spacer?: boolean;
		gap?: number;
		top?: Snippet;
		title?: Snippet;
		description?: Snippet;
		children: Snippet;
	}

	const { spacer = false, gap = 16, top, title, description, children }: Props = $props();
</script>

<div class="settings-section" style="gap: {pxToRem(gap)}rem">
	{#if top}
		{@render top?.()}
	{/if}

	{#if title || description}
		<div class="description">
			{#if title}
				<h2 class="text-15 text-bold">
					{@render title?.()}
				</h2>
			{/if}
			{#if description}
				<p class="text-12 text-body">
					{@render description?.()}
				</p>
			{/if}
		</div>
	{/if}

	{@render children?.()}

	{#if spacer}
		<Spacer />
	{/if}
</div>

<style>
	.settings-section {
		display: flex;
		flex-direction: column;
	}

	.description {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.description h2 {
		color: var(--clr-text-1);
	}

	.description p {
		color: var(--clr-text-2);
	}
</style>
