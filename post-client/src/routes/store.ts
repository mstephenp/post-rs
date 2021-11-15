import { writable, derived } from 'svelte/store';

export const postData = writable([]);

export const posts = derived(postData, ($postData) => {
    return $postData.map(post => post.content)
});