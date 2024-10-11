<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <div v-if="users.length > 0" class="flex items-center">
        <div>{{ props.pretext }}&nbsp;</div>
        <a :href="`https://github.com/${props.owner}/${props.repo}`"
            class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-1 px-2 rounded-l inline-flex items-center">
            <svg class="fill-current w-4 h-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512">
                <path
                    d="M316.9 18C311.6 7 300.4 0 288.1 0s-23.4 7-28.8 18L195 150.3 51.4 171.5c-12 1.8-22 10.2-25.7 21.7s-.7 24.2 7.9 32.7L137.8 329 113.2 474.7c-2 12 3 24.2 12.9 31.3s23 8 33.8 2.3l128.3-68.5 128.3 68.5c10.8 5.7 23.9 4.9 33.8-2.3s14.9-19.3 12.9-31.3L438.5 329 542.7 225.9c8.6-8.5 11.7-21.2 7.9-32.7s-13.7-19.9-25.7-21.7L381.2 150.3 316.9 18z" />
            </svg>
            <span>Stars</span>
        </a>
        <a :href="`https://github.com/${props.owner}/${props.repo}`"
            class="bg-transparent hover:bg-gray-400 text-gray-800 font-semibold hover:text-white py-[0.18rem] px-2 border border-gray-300 hover:border-transparent rounded-r">
            {{ users.length }}
        </a>
        <div>&nbsp;{{ props.posttext }}&nbsp;</div>
    </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue';

/**
 * Interface representing a GitHub user.
 *
 * Properties are marked as optional to accommodate for incomplete data from the GitHub API.
 */
export interface GitHubUser {
    login?: string;
    id?: number;
    node_id?: string;
    avatar_url?: string;
    gravatar_id?: string;
    url?: string;
    html_url?: string;
    followers_url?: string;
    following_url?: string;
    gists_url?: string;
    starred_url?: string;
    subscriptions_url?: string;
    organizations_url?: string;
    repos_url?: string;
    events_url?: string;
    received_events_url?: string;
    type?: string;
    site_admin?: boolean;
}

/**
 * Type alias for an array of GitHub users.
 */
export type GitHubUsers = GitHubUser[];
export interface StargazersResponse {
    users: GitHubUsers;
    isLastPage: boolean;
}

const props = withDefaults(defineProps<{
    /**
     * The repository owner (defaults to 'sctg-development').
     */
    owner?: string;
    /**
     * The repository name (defaults to 'sctgdesk-server').
     */
    repo?: string;
    /**
     * The text to display before the stargazers count (defaults to 'Give us a star').
     */
    pretext?: string;
    /**
     * The text to display after the stargazers count (defaults to 'on GitHub').
     */
    posttext?: string;
}>(),
    {
        owner: 'sctg-development',
        repo: 'sctgdesk-server',
        pretext: 'Give us a star',
        posttext: 'on GitHub'
    })

const users = ref<GitHubUsers>([]);

/**
 * Fetch a page of stargazers
 * @param owner The owner of the repository
 * @param repo The repository name
 * @param page The page number to fetch
 * @returns The stargazers response
 */
async function getStargazersPage(owner: string, repo: string, page: number): Promise<StargazersResponse> {
    const response = await fetch(`https://api.github.com/repos/${owner}/${repo}/stargazers?per_page=100&page=${page}`, {
        headers: {
            'Accept': 'application/vnd.github+json',
            'X-GitHub-Api-Version': '2022-11-28'
        }
    });
    const githubUsers: GitHubUsers = await response.json();
    const isNotLastPage = response.headers.get('Link')?.toLowerCase().includes('rel="next"') ?? true; // if there is no 'next' link, then it is the last page
    return { users: githubUsers, isLastPage: !isNotLastPage } as StargazersResponse;
}

/**
 * Fetch all stargazers for the repository
 * @param owner The owner of the repository
 * @param repo The repository name
 * @returns All stargazers
 */
async function fetchAllStargazers(owner: string, repo: string): Promise<GitHubUsers> {
    const allUsers: GitHubUsers = [];
    let page = 1;
    let isLastPage = false;
    // Continuously fetch pages until we've reached the last page.
    while (!isLastPage) {
        // Fetch the current page of stargazers.
        const { users: pageUsers, isLastPage: pageIsLastPage } = await getStargazersPage(owner, repo, page);

        // Add the stargazers from this page to the total array.
        allUsers.push(...pageUsers);

        // Update the last page tracker.
        isLastPage = pageIsLastPage;

        // Move on to the next page.
        page++;
    }
    return allUsers;
}

onMounted(() => {
    fetchAllStargazers(props.owner, props.repo).then((allUsers) => {
        users.value = allUsers;
    });
});
</script>
