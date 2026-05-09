<script>
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { deletePost } from "$lib/api";
  import { DeleteButton } from "$lib/components";
  import { invalidateAll } from "$app/navigation";
  import { likePost } from "$lib/api";

  /** @type {{ post: Post }} */
  const { post } = $props();

  let deleting = $state(false);
  let liking = $state(false);
  let liked = $derived(false);

  function likedKey() {
    return `liked_post_${post.id}`;
  }

  $effect(() => {
    liked = localStorage.getItem(likedKey()) === "true";
  });

  async function handleLike() {
    if (liked || liking) {
      alert("이미 좋아요를 눌렀습니다.");
      return;
    }

    liking = true;

    try {
      await likePost({ id: post.id });
      localStorage.setItem(likedKey(), "true");
      liked = true;

      await invalidateAll();
    } catch {
      alert("좋아요 처리에 실패했습니다.");
    } finally {
      liking = false;
    }
  }

  async function handleDelete() {
    const password = prompt("비밀번호를 입력하세요");
    if (password === null) return;

    deleting = true;
    try {
      await deletePost({ id: post.id, password });
      goto(resolve("/"), { invalidateAll: true });
    } catch (errRes) {
      if (!(errRes instanceof Response)) throw errRes;

      switch (errRes.status) {
        case 403:
          alert("비밀번호가 일치하지 않습니다");
          break;
        case 404:
          alert("게시글이 존재하지 않습니다");
          break;
        default:
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
  <button
  class="like-btn {liked ? 'liked' : ''}"
  onclick={handleLike}
  disabled={liked || liking}
>
    ♡ {post.likeCount ?? ""}
  </button>

  {#if !post.category.readonly}
    <DeleteButton onclick={handleDelete} disabled={deleting} />
  {/if}
</div>
</article>

<style>
  .like-btn {
  padding: 8px 16px;
  border-radius: 20px;
  border: 1px solid #ff6b6b;
  background-color: white;
  color: #ff6b6b;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.like-btn:hover {
  background-color: #ff6b6b;
  color: white;
  transform: scale(1.05);
}

.like-btn.liked {
  background-color: #ff6b6b;
  color: white;
  border: none;
}

.like-btn:disabled {
  cursor: not-allowed;
}

.like-btn:active {
  transform: scale(0.95);
}
</style>
