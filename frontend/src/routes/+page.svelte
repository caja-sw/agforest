<script>
  import { resolve } from "$app/paths";

  const { data } = $props();
  const { categories, currentCategory, pages, currentPage, posts } =
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
  <nav class="flex justify-between glass p-4">
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

  {#if currentCategory !== null}
    <section class="grid gap-6 glass p-6">
      <h1 class="text-2xl leading-none">{currentCategory.name}</h1>

      <section class="card p-4">
        <ul>
          {#each posts as post (post.id)}
            <li>
              <a href={resolve("/post/[id]", { id: String(post.id) })}>
                <section
                  class="grid grid-cols-[minmax(5rem,1fr)_repeat(3,10rem)] hover:bg-text/5"
                >
                  <h1
                    class="w-full place-self-start overflow-hidden text-ellipsis whitespace-nowrap"
                  >
                    {post.title}
                  </h1>
                  <span
                    class="w-full place-self-end overflow-hidden text-ellipsis whitespace-nowrap"
                    >{post.author.name}</span
                  >
                  <span class="place-self-start text-text-muted"
                    >#{post.author.hash.slice(0, 6)}</span
                  >
                  <time class="place-self-end"
                    >{formatCreatedAt(new Date(post.createdAt))}</time
                  >
                </section>
              </a>
            </li>
          {/each}
        </ul>
      </section>
    </section>

    <nav class="glass p-4">
      <ul class="flex flex-row justify-center gap-5">
        {#each pages as page (page)}
          <li>
            <!-- eslint-disable svelte/no-navigation-without-resolve -->
            <a
              class={[
                "grid size-10 place-items-center rounded-full bg-bg font-bold text-text-muted outline",
                page === currentPage && "bg-primary text-bg",
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
  {/if}
</div>
