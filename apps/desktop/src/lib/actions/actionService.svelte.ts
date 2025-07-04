import { invalidatesList, ReduxTag } from '$lib/state/tags';
import type { TreeChange } from '$lib/hunks/change';
import type { BackendApi, ClientState } from '$lib/state/clientState.svelte';

type ChatMessage = {
	type: 'user' | 'assistant';
	content: string;
};

export class ActionService {
	private api: ReturnType<typeof injectEndpoints>;

	constructor(backendApi: BackendApi) {
		this.api = injectEndpoints(backendApi);
	}

	get autoCommit() {
		return this.api.endpoints.autoCommit.useMutation();
	}

	get branchChanges() {
		return this.api.endpoints.autoBranchChanges.useMutation();
	}

	get absorb() {
		return this.api.endpoints.absorb.useMutation();
	}

	get freestyle() {
		return this.api.endpoints.freestyle.useMutation();
	}
}

function injectEndpoints(api: ClientState['backendApi']) {
	return api.injectEndpoints({
		endpoints: (build) => ({
			autoCommit: build.mutation<void, { projectId: string; changes: TreeChange[] }>({
				query: ({ projectId, changes }) => ({
					command: 'auto_commit',
					params: { projectId, changes },
					actionName: 'Figure out where to commit the given changes'
				}),
				invalidatesTags: [
					invalidatesList(ReduxTag.Stacks),
					invalidatesList(ReduxTag.StackDetails),
					invalidatesList(ReduxTag.WorktreeChanges)
				]
			}),
			autoBranchChanges: build.mutation<void, { projectId: string; changes: TreeChange[] }>({
				query: ({ projectId, changes }) => ({
					command: 'auto_branch_changes',
					params: { projectId, changes },
					actionName: 'Create a branch for the given changes'
				}),
				invalidatesTags: [
					invalidatesList(ReduxTag.Stacks),
					invalidatesList(ReduxTag.StackDetails),
					invalidatesList(ReduxTag.WorktreeChanges)
				]
			}),
			absorb: build.mutation<void, { projectId: string; changes: TreeChange[] }>({
				query: ({ projectId, changes }) => ({
					command: 'absorb',
					params: { projectId, changes },
					actionName: 'Absorb changes into the best matching branch and commit'
				}),
				invalidatesTags: [
					invalidatesList(ReduxTag.Stacks),
					invalidatesList(ReduxTag.StackDetails),
					invalidatesList(ReduxTag.WorktreeChanges)
				]
			}),
			freestyle: build.mutation<
				string,
				{ projectId: string; chatMessages: ChatMessage[]; model: string | null }
			>({
				query: ({ projectId, chatMessages, model }) => ({
					command: 'freestyle',
					params: { projectId, chatMessages, model },
					actionName: 'Perform a freestyle action based on the given prompt'
				}),
				invalidatesTags: [
					invalidatesList(ReduxTag.Stacks),
					invalidatesList(ReduxTag.StackDetails),
					invalidatesList(ReduxTag.WorktreeChanges)
				]
			})
		})
	});
}
