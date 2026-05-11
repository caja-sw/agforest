<script>
  import CommentListCard from "./CommentListCard.svelte";
  import CommentWriteCard from "./CommentWriteCard.svelte";
  import PostCard from "./PostCard.svelte";

  const { data } = $props();
</script>

<div class="grid gap-4 lg:grid-cols-[4fr_6fr]">
  <div class="col-span-full">
    <PostCard post={data.post} />
  </div>
  <div>
    <CommentWriteCard postId={data.post.id} />
  </div>
  <div>
    <CommentListCard comments={data.post.comments} />
  </div>
</div>
