<script>
  import { resolve } from "$app/paths";

  const { data } = $props();
  const { categories, currentCategory, currentPage, maxPage, pages, posts } =
    $derived(data);

  /**
   * @param {Date} date
   * @returns {string}
   */
  function formatCreatedAt(date) {
    const now = new Date();

    const isToday =
      date.getFullYear() === now.getFullYear() &&
      date.getMonth() === now.getMonth() &&
      date.getDate() === now.getDate();

    if (isToday) {
      return date.toLocaleTimeString(undefined, {
        hour: "2-digit",
        minute: "2-digit",
      });
    }

    return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    });
  }
</script>

<div class="grid gap-4">
  <nav class="glass flex justify-between p-4">
    <ul class="flex">
      {#each categories as category (category.id)}
        <li>
          <!-- eslint-disable svelte/no-navigation-without-resolve -->
          <a
            class="block p-2 leading-none"
            href={`${resolve("/")}?c=${category.id}`}>{category.name}</a
          >
          <!-- eslint-enable svelte/no-navigation-without-resolve -->
        </li>
      {/each}
    </ul>

    <div class="flex">
      <a class="block p-2 leading-none" href={resolve("/write")}>게시글 쓰기</a>
    </div>
  </nav>

  <section class="glass grid gap-6 py-6">
    <header class="flex items-center justify-between px-6">
      <h1 class="text-2xl leading-none">{currentCategory.name}</h1>
      <span class="text-text-muted">{currentPage}/{maxPage} 페이지</span>
    </header>

    <div class="card p-4 md:mx-6">
      {#if posts.length > 0}
        <ul>
          {#each posts as post (post.id)}
            <li>
              <a href={resolve("/post/[id]", { id: String(post.id) })}>
                <section
                  class="hover:bg-text/5 grid grid-cols-[1fr_auto] gap-4 py-1"
                >
                  <div>
                    <h1 class="grid">
                      <span
                        class="overflow-hidden text-ellipsis whitespace-nowrap"
                        >{post.title}</span
                      >
                    </h1>
                    <div class="text-text-muted flex flex-wrap gap-x-4">
                      <span class="grid grid-cols-[1fr_auto]">
                        <span
                          class="overflow-hidden text-ellipsis whitespace-nowrap"
                          >{post.author.name}</span
                        >
                        <span>#{post.author.hash.slice(0, 6)}</span>
                      </span>
                      <time>{formatCreatedAt(new Date(post.createdAt))}</time>
                    </div>
                  </div>
                  <div class="w-8 place-self-center text-center">
                    <span
                      >{post.commentCount < 100
                        ? post.commentCount
                        : "99+"}</span
                    >
                  </div>
                </section>
              </a>
            </li>
          {/each}
        </ul>
      {:else}
        <p>게시글이 없습니다</p>
      {/if}
    </div>
  </section>

  <nav class="glass p-4">
    <ul class="flex flex-row justify-center gap-5">
      {#each pages as page (page)}
        <li>
          <!-- eslint-disable svelte/no-navigation-without-resolve -->
          <a
            class={[
              "grid size-10 place-items-center rounded-full font-bold outline",
              page === currentPage
                ? "bg-primary text-bg"
                : "bg-bg text-text-muted",
            ]}
            href={`${resolve("/")}?c=${currentCategory.id}&p=${page}`}
          >
            <span>{page}</span>
          </a>
          <!-- eslint-enable svelte/no-navigation-without-resolve -->
        </li>
      {/each}
    </ul>
  </nav>
</div>
