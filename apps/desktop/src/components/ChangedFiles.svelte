<script lang="ts">
	import Drawer from '$components/Drawer.svelte';
	import FileList from '$components/FileList.svelte';
	import FileListMode from '$components/FileListMode.svelte';
	import Resizer from '$components/Resizer.svelte';
	import emptyFolderSvg from '$lib/assets/empty-state/empty-folder.svg?raw';
	import { INTELLIGENT_SCROLLING_SERVICE } from '$lib/intelligentScrolling/service';
	import { ID_SELECTION } from '$lib/selection/idSelection.svelte';
	import { inject } from '@gitbutler/shared/context';
	import { Badge, EmptyStatePlaceholder, LineStats } from '@gitbutler/ui';

	import { untrack, type ComponentProps } from 'svelte';
	import type { ConflictEntriesObj } from '$lib/files/conflicts';
	import type { TreeChange, TreeStats } from '$lib/hunks/change';
	import type { SelectionId } from '$lib/selection/key';

	type Props = {
		projectId: string;
		stackId?: string;
		selectionId: SelectionId;
		changes: TreeChange[];
		title: string;
		stats?: TreeStats;
		active?: boolean;
		conflictEntries?: ConflictEntriesObj;
		draggableFiles?: boolean;
		grow?: boolean;
		noshrink?: boolean;
		bottomBorder?: boolean;
		resizer?: Partial<ComponentProps<typeof Resizer>>;
		autoselect?: boolean;
		ancestorMostConflictedCommitId?: string;
		ontoggle?: (collapsed: boolean) => void;
	};

	const {
		projectId,
		stackId,
		selectionId,
		changes,
		title,
		stats,
		active,
		conflictEntries,
		draggableFiles,
		grow,
		noshrink,
		bottomBorder,
		resizer,
		autoselect,
		ancestorMostConflictedCommitId,
		ontoggle
	}: Props = $props();

	const idSelection = inject(ID_SELECTION);
	const intelligentScrollingService = inject(INTELLIGENT_SCROLLING_SERVICE);

	let listMode: 'list' | 'tree' = $state('tree');

	$effect(() => {
		if (selectionId) {
			const selection = idSelection.getById(selectionId);
			// Prevent effect from running when `changes` updates.
			untrack(() => {
				if (autoselect && selection.entries.size === 0) {
					const firstChange = changes.at(0);
					if (firstChange) {
						idSelection.set(firstChange.path, selectionId, 0);
					}
				}
			});
		}
	});
</script>

<Drawer {grow} {ontoggle} {resizer} {noshrink}>
	{#snippet header()}
		<h4 class="text-14 text-semibold truncate">{title}</h4>
		<div class="text-11 header-stats">
			<Badge>{changes.length}</Badge>
			{#if stats}
				<LineStats linesAdded={stats.linesAdded} linesRemoved={stats.linesRemoved} />
			{/if}
		</div>
	{/snippet}
	{#snippet extraActions()}
		<FileListMode bind:mode={listMode} persist="committed" />
	{/snippet}

	<div class="filelist-wrapper" class:bottom-border={bottomBorder}>
		{#if changes.length > 0}
			<FileList
				{selectionId}
				{projectId}
				{stackId}
				{changes}
				{listMode}
				{active}
				{conflictEntries}
				{draggableFiles}
				{ancestorMostConflictedCommitId}
				onselect={() => {
					if (stackId) {
						intelligentScrollingService.show(projectId, stackId, 'diff');
					}
				}}
			/>
		{:else}
			<EmptyStatePlaceholder image={emptyFolderSvg} width={180} gap={4}>
				{#snippet caption()}
					No files changed
				{/snippet}
			</EmptyStatePlaceholder>
		{/if}
	</div>
</Drawer>

<style lang="postcss">
	.header-stats {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.filelist-wrapper {
		display: flex;
		flex-direction: column;
		background-color: var(--clr-bg-1);

		&.bottom-border {
			border-bottom: 1px solid var(--clr-border-2);
		}
	}
</style>
