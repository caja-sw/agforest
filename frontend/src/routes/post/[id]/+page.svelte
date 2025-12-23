<script>
  import CommentListCard from "./CommentListCard.svelte";
  import CommentWriteCard from "./CommentWriteCard.svelte";
  import PostCard from "./PostCard.svelte";

  const { data } = $props();
  const { post } = $derived(data);
</script>

<div class="grid gap-4 lg:grid-cols-[0.4fr_0.6fr]">
  <div class="col-span-full">
    <PostCard {post} />
  </div>
  <div>
    <CommentWriteCard postId={post.id} />
  </div>
  <div>
    <CommentListCard comments={post.comments} />
  </div>
</div>
