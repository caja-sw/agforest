<script>
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { deletePost } from "$lib/api";
  import { DeleteButton } from "$lib/components";

  const { post } = $props();

  async function handleDelete() {
    const password = prompt("비밀번호를 입력하세요");
    if (password === null) return;

    try {
      await deletePost({ id: post.id, password });
      goto(resolve("/"));
    } catch (errRes) {
      if (!(errRes instanceof Response)) throw errRes;

      switch (errRes.status) {
        case 403: {
          alert("비밀번호가 일치하지 않습니다");
          break;
        }
        case 404: {
          alert("게시글이 존재하지 않습니다");
          break;
        }
        default: {
          alert("알 수 없는 오류가 발생했습니다");
        }
      }
    }
  }
</script>

<article class="post glass">
  <header>
    <h1>{post.title}</h1>
    <div class="meta">
      <p>
        <span class="name">{post.author.name}</span>
        <span>#{post.author.hash.slice(0, 6)}</span>
      </p>
      <time>{new Date(post.createdAt).toLocaleString()}</time>
    </div>
  </header>

  <hr />

  <p class="content">{post.content}</p>

  <div class="actions">
    <DeleteButton onclick={handleDelete} />
  </div>
</article>

<style>
  .post {
    padding: 24px;
  }

  header {
    display: grid;
    gap: 8px;
  }

  h1 {
    font-size: 2rem;
    line-height: 1;
  }

  .meta {
    display: flex;
    justify-content: space-between;
    gap: 16px;
    color: var(--color-text-muted);
  }

  .name {
    color: var(--color-text);
  }

  hr {
    margin: 8px 0;
  }

  .content {
    white-space: pre-wrap;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
  }
</style>
