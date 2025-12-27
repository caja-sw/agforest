<script>
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { deletePost } from "$lib/api";
  import { DeleteButton } from "$lib/components";

  /** @type {{ post: Post }} */
  const { post } = $props();

  let deleting = $state(false);

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

  {#if !post.category.readonly}
    <div class="place-self-end">
      <DeleteButton onclick={handleDelete} disabled={deleting} />
    </div>
  {/if}
</article>
