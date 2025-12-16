<script>
  import { deleteComment } from "$lib/api";
  import DeleteButton from "$lib/components/DeleteButton.svelte";

  /** @type {{ comments: { id: number, author: { name: string, hash: string }, content: string, createdAt: string }[], onupdate: () => void }} */
  const { comments, onupdate } = $props();

  /** @param {number} id  */
  async function handleDelete(id) {
    const password = prompt("비밀번호를 입력하세요");
    if (password === null) return;

    try {
      await deleteComment({ id, password });
      onupdate();
    } catch (errRes) {
      if (!(errRes instanceof Response)) throw errRes;

      switch (errRes.status) {
        case 403: {
          alert("비밀번호가 일치하지 않습니다");
          break;
        }
        case 404: {
          alert("댓글이 존재하지 않습니다");
          break;
        }
        default: {
          alert("알 수 없는 오류가 발생했습니다");
        }
      }
    }
  }
</script>

<section class="container glass">
  <h1 class="header">댓글</h1>
  <ul>
    {#each comments as comment (comment.id)}
      <li>
        <article class="card">
          <header>
            <p>
              <span class="name">{comment.author.name}</span>
              <span>#{comment.author.hash.slice(0, 6)}</span>
            </p>
            <time>{new Date(comment.createdAt).toLocaleString()}</time>
          </header>

          <p>{comment.content}</p>

          <div class="actions">
            <DeleteButton onclick={() => handleDelete(comment.id)} />
          </div>
        </article>
      </li>
    {/each}
  </ul>
</section>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 24px;
  }

  .header {
    font-size: 1.2rem;
  }

  ul {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  article {
    display: grid;
    padding: 16px;
  }

  header {
    display: flex;
    justify-content: space-between;
    gap: 16px;
    color: var(--color-text-muted);
  }

  .name {
    color: var(--color-text);
  }

  .actions {
    place-self: end;
  }
</style>
