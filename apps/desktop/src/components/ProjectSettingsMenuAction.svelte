<script lang="ts">
	import { goto } from '$app/navigation';
	import { vscodePath } from '$lib/project/project';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { projectSettingsPath } from '$lib/routes/routes.svelte';
	import { historyPath } from '$lib/routes/routes.svelte';
	import { SETTINGS } from '$lib/settings/userSettings';
	import { SHORTCUT_SERVICE } from '$lib/shortcuts/shortcutService';
	import { getEditorUri, openExternalUrl, showFileInFolder } from '$lib/utils/url';
	import { inject } from '@gitbutler/shared/context';
	import { mergeUnlisten } from '@gitbutler/ui/utils/mergeUnlisten';

	const { projectId }: { projectId: string } = $props();

	const projectsService = inject(PROJECTS_SERVICE);

	const userSettings = inject(SETTINGS);
	const shortcutService = inject(SHORTCUT_SERVICE);

	$effect(() =>
		mergeUnlisten(
			shortcutService.on('project-settings', () => {
				goto(projectSettingsPath(projectId));
			}),
			shortcutService.on('history', () => {
				goto(historyPath(projectId));
			}),
			shortcutService.on('open-in-vscode', async () => {
				const project = await projectsService.fetchProject(projectId);
				if (!project) {
					throw new Error(`Project not found: ${projectId}`);
				}
				openExternalUrl(
					getEditorUri({
						schemeId: $userSettings.defaultCodeEditor.schemeIdentifer,
						path: [vscodePath(project.path)],
						searchParams: { windowId: '_blank' }
					})
				);
			}),
			shortcutService.on('show-in-finder', async () => {
				const project = await projectsService.fetchProject(projectId);
				if (!project) {
					throw new Error(`Project not found: ${projectId}`);
				}
				// Show the project directory in the default file manager (cross-platform)
				await showFileInFolder(project.path);
			})
		)
	);
</script>
