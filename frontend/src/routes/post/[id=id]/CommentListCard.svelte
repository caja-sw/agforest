<script>
  import { invalidateAll } from "$app/navigation";
  import { deleteComment } from "$lib/api";
  import { DeleteButton } from "$lib/components";
  import { likeComment } from "$lib/api";

  /** @type {{ comments: CommentListItem[] }} */
  const { comments } = $props();

  let deleting = $state(false);
  let likingId = $state(null);
  let likedMap = $state({});

  function likedKey(id) {
    return `liked_comment_${id}`;
  }

  $effect(() => {
    const map = {};
    for (const comment of comments) {
      map[comment.id] =
        localStorage.getItem(likedKey(comment.id)) === "true";
    }
    likedMap = map;
  });

  async function handleLike(commentId) {
    if (likedMap[commentId] || likingId === commentId) {
      alert("이미 좋아요를 눌렀습니다.");
      return;
    }

    likingId = commentId;

    try {
      await likeComment({ id: commentId });

      localStorage.setItem(likedKey(commentId), "true");
      likedMap = { ...likedMap, [commentId]: true };

      await invalidateAll();
    } catch {
      alert("댓글 좋아요 처리에 실패했습니다.");
    } finally {
      likingId = null;
    }
  }

  /** @param {number} id  */
  async function handleDelete(id) {
    const password = prompt("비밀번호를 입력하세요");
    if (password === null) return;

    deleting = true;
    try {
      await deleteComment({ id, password });
      invalidateAll();
    } catch (errRes) {
      if (!(errRes instanceof Response)) throw errRes;

      switch (errRes.status) {
        case 403:
          alert("비밀번호가 일치하지 않습니다");
          break;
        case 404:
          alert("댓글이 존재하지 않습니다");
          break;
        default:
          alert("알 수 없는 오류가 발생했습니다");
      }
    } finally {
      deleting = false;
    }
  }
</script>

<section class="glass grid gap-2 p-6">
  <h1 class="text-xl">댓글</h1>
  {#if comments.length > 0}
    <ul class="grid gap-4">
      {#each comments as comment (comment.id)}
        <li>
          <article class="card grid p-4">
            <header
              class="text-text-muted flex flex-wrap justify-between gap-x-4"
            >
              <span
                >{comment.author.name}#{comment.author.hash.slice(0, 6)}</span
              >
              <time>{new Date(comment.createdAt).toLocaleString()}</time>
            </header>

            <p class="whitespace-pre-wrap">{comment.content}</p>

            <div class="place-self-end">
              <button
                class="like-btn"
                class:liked={likedMap[comment.id]}
                onclick={() => handleLike(comment.id)}
                disabled={likedMap[comment.id] || likingId === comment.id}
              >
                ♡ {comment.likeCount ?? ""}
              </button>
              <DeleteButton
                onclick={() => handleDelete(comment.id)}
                disabled={deleting}
              />
            </div>
          </article>
        </li>
      {/each}
    </ul>
  {:else}
    <p>댓글이 없습니다</p>
  {/if}
</section>

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