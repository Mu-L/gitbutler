import { ghQuery } from '$lib/forge/github/ghQuery';
import { providesList, ReduxTag } from '$lib/state/tags';
import { InjectionToken } from '@gitbutler/shared/context';
import type { Tauri } from '$lib/backend/tauri';
import type { GitHubApi } from '$lib/state/clientState.svelte';
import type { RestEndpointMethodTypes } from '@octokit/rest';

export const GITHUB_USER_SERVICE = new InjectionToken<GitHubUserService>('GitHubUserService');

type IsAuthenticated = RestEndpointMethodTypes['users']['getAuthenticated']['response']['data'];

type Verification = {
	user_code: string;
	device_code: string;
};

export class GitHubUserService {
	private api: ReturnType<typeof injectEndpoints>;

	constructor(
		private tauri: Tauri,
		gitHubApi: GitHubApi
	) {
		this.api = injectEndpoints(gitHubApi);
	}

	async fetchGitHubLogin() {
		return await this.api.endpoints.getAuthenticated.fetch();
	}

	async initDeviceOauth() {
		return await this.tauri.invoke<Verification>('init_device_oauth');
	}

	async checkAuthStatus(params: { deviceCode: string }) {
		return await this.tauri.invoke<string>('check_auth_status', params);
	}
}

function injectEndpoints(api: GitHubApi) {
	return api.injectEndpoints({
		endpoints: (build) => ({
			getAuthenticated: build.query<IsAuthenticated, void>({
				queryFn: async (_, api) =>
					await ghQuery({
						domain: 'users',
						action: 'getAuthenticated',
						extra: api.extra
					}),
				providesTags: [providesList(ReduxTag.PullRequests)]
			})
		})
	});
}
