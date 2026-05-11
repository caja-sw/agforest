<script>
  import { goto, invalidateAll } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { deletePost, likePost, unlikePost } from "$lib/api";
  import { DeleteButton, LikeButton } from "$lib/components";
  import { onMount } from "svelte";

  /** @type {{ post: Post }} */
  const { post } = $props();

  let deleting = $state(false);
  let liking = $state(false);
  let liked = $state(false);
  const likedKey = $derived(`liked_post_${post.id}`);

  onMount(() => {
    liked = localStorage.getItem(likedKey) !== null;
  });

  $effect(() => {
    if (liked) {
      localStorage.setItem(likedKey, "");
    } else {
      localStorage.removeItem(likedKey);
    }
  });

  async function handleLike() {
    if (liking) {
      return;
    }
    liking = true;

    try {
      if (liked) {
        await unlikePost({ id: post.id });
        liked = false;
      } else {
        await likePost({ id: post.id });
        liked = true;
      }
    } catch {
      alert("좋아요 처리에 실패했습니다.");
    } finally {
      liking = false;
      await invalidateAll();
    }
  }

  async function handleDelete() {
    if (deleting) {
      return;
    }
    deleting = true;

    try {
      const password = prompt("비밀번호를 입력하세요");
      if (password === null) {
        return;
      }

      await deletePost({ id: post.id, password });
      goto(resolve("/[id=id]", { id: String(post.category.id) }));
    } catch (errRes) {
      if (!(errRes instanceof Response)) {
        throw errRes;
      }

      if (errRes.status === 403) {
        alert("비밀번호가 일치하지 않습니다");
      } else if (errRes.status === 404) {
        alert("게시글이 존재하지 않습니다");
      } else {
        alert("알 수 없는 오류가 발생했습니다");
      }
    } finally {
      deleting = false;
    }
  }
</script>

<article class="glass p-6">
  <header class="grid gap-2">
    <div>
      <span class="text-text-muted text-sm">{post.category.name}</span>
      <h1 class="text-3xl leading-none">{post.title}</h1>
    </div>
    <div class="text-text-muted flex flex-wrap justify-between gap-x-4">
      <span>{post.author.name}#{post.author.hash.slice(0, 6)}</span>
      <time>{new Date(post.createdAt).toLocaleString()}</time>
    </div>
  </header>

  <hr class="my-2" />

  <p class="whitespace-pre-wrap">{post.content}</p>

  <div class="mt-4 flex justify-end gap-2">
    <LikeButton {liked} count={post.likeCount} onclick={handleLike} disabled={liking} />

    {#if !post.category.readonly}
      <DeleteButton onclick={handleDelete} disabled={deleting} />
    {/if}
  </div>
</article>
