<script>
  import { invalidateAll } from "$app/navigation";
  import { deleteComment } from "$lib/api";
  import { DeleteButton } from "$lib/components";

  /** @type {{ comments: { id: number, author: { name: string, hash: string }, content: string, createdAt: string }[] }} */
  const { comments } = $props();

  let deleting = $state(false);

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
