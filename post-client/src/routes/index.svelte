<script>
    import { onMount } from "svelte";
    import { postData, posts } from "./store";

    onMount(async () => {
        fetch("http://127.0.0.1:3000/posts")
        .then(response => response.json())
        .then(data => {
            postData.set(data)
        }).catch(error => {
            console.log(error);
            return [];
        })
    })

</script>

<svelte:head>
    <title>post-client</title>
</svelte:head>

<h1>Posts</h1>
<ul>
    {#each $posts as post}
        <li>{post}</li>
    {/each}
</ul>