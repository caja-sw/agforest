<script>
  import { invalidateAll } from "$app/navigation";
  import CommentListCard from "./CommentListCard.svelte";
  import CommentWriteCard from "./CommentWriteCard.svelte";
  import PostCard from "./PostCard.svelte";

  const { data } = $props();
  const { post, comments } = $derived(data);
</script>

<div class="container">
  <div class="content">
    <div class="post">
      <PostCard {post} />
    </div>

    <div class="comment-write">
      <CommentWriteCard postId={post.id} onupdate={invalidateAll} />
    </div>

    <div class="comment-list">
      <CommentListCard {comments} onupdate={invalidateAll} />
    </div>
  </div>
</div>

<style>
  .container {
    container-type: inline-size;
  }

  .content {
    display: grid;
    grid-template-areas:
      "post         "
      "comment-write"
      "comment-list ";
    gap: 16px;
  }

  @container (min-width: calc(75rem - 64px * 2)) {
    .content {
      grid-template-columns: 0.4fr 0.6fr;
      grid-template-areas:
        "post          post        "
        "comment-write comment-list";
    }
  }

  .post {
    grid-area: post;
  }

  .comment-write {
    grid-area: comment-write;
  }

  .comment-list {
    grid-area: comment-list;
  }
</style>
