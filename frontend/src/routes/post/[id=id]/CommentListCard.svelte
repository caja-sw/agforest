<script>
  import CommentCard from "./CommentCard.svelte";

  /** @type {{ comments: Comment[] }} */
  const { comments } = $props();
</script>

<section class="glass grid gap-2 p-6">
  <h1 class="text-xl">댓글</h1>
  {#if comments.length > 0}
    <ul class="grid gap-4">
      {#each comments as comment (comment.id)}
        <li>
          <CommentCard {comment} />
        </li>
      {/each}
    </ul>
  {:else}
    <p>댓글이 없습니다</p>
  {/if}
</section>
