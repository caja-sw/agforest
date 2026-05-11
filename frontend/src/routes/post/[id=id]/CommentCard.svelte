<script>
  import { invalidateAll } from "$app/navigation";
  import { deleteComment, likeComment, unlikeComment } from "$lib/api";
  import { DeleteButton, LikeButton } from "$lib/components";
  import { onMount } from "svelte";

  /** @type {{ comment: Comment }} */
  const { comment } = $props();

  let deleting = $state(false);
  let liking = $state(false);
  let liked = $state(false);
  const likedKey = $derived(`liked_comment_${comment.id}`);

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
        await unlikeComment({ id: comment.id });
        liked = false;
      } else {
        await likeComment({ id: comment.id });
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

      await deleteComment({ id: comment.id, password });
    } catch (errRes) {
      if (!(errRes instanceof Response)) {
        throw errRes;
      }

      if (errRes.status === 403) {
        alert("비밀번호가 일치하지 않습니다");
      } else if (errRes.status === 404) {
        alert("댓글이 존재하지 않습니다");
      } else {
        alert("알 수 없는 오류가 발생했습니다");
      }
    } finally {
      deleting = false;
      await invalidateAll();
    }
  }
</script>

<article class="card grid p-4">
  <header class="text-text-muted flex flex-wrap justify-between gap-x-4">
    <span>{comment.author.name}#{comment.author.hash.slice(0, 6)}</span>
    <time>{new Date(comment.createdAt).toLocaleString()}</time>
  </header>

  <p class="whitespace-pre-wrap">{comment.content}</p>

  <div class="flex gap-2 place-self-end">
    <LikeButton {liked} count={comment.likeCount} onclick={handleLike} disabled={liking} />
    <DeleteButton onclick={handleDelete} disabled={deleting} />
  </div>
</article>
