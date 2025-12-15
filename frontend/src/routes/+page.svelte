<script>
  import { resolve } from "$app/paths";

  const { data } = $props();
  const { categories, currentCategory, pages, currentPage, posts } = $derived(data);

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

<div class="container">
  <nav class="navbar glass">
    <ul class="categories">
      {#each categories as category (category.id)}
        <li>
          <!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
          <a href={`${resolve("/")}?c=${category.id}`}>{category.name}</a>
        </li>
      {/each}
    </ul>

    <div class="actions">
      <a href={resolve("/write")}>게시글 쓰기</a>
    </div>
  </nav>

  {#if currentCategory !== null}
    <section class="content glass">
      <h1>{currentCategory.name}</h1>

      <section class="posts card">
        <ul>
          {#each posts as post (post.id)}
            <li>
              <section>
                <a class="post" href={resolve("/post/[id]", { id: String(post.id) })}>
                  <h2 class="title">{post.title}</h2>
                  <span class="name">{post.author.name}</span>
                  <span class="hash">#{post.author.hash}</span>
                  <time class="created-at">{formatCreatedAt(new Date(post.createdAt))}</time>
                </a>
              </section>
            </li>
          {/each}
        </ul>
      </section>
    </section>

    <nav class="pagination glass">
      <ul>
        {#each pages as page (page)}
          <li>
            <!-- eslint-disable svelte/no-navigation-without-resolve -->
            <a
              href={`${resolve("/")}?c=${currentCategory.id}&p=${page}`}
              class:current={page === currentPage}
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

<style>
  .container {
    display: grid;
    gap: 16px;
  }

  .navbar {
    display: flex;
    justify-content: space-between;
    padding: 16px;
  }

  .categories,
  .actions {
    display: flex;
  }

  .categories > li > a,
  .actions > a {
    display: block;
    padding: 8px;
    line-height: 1;
  }

  .content {
    display: grid;
    gap: 24px;
    padding: 24px;
  }

  .content > h1 {
    font-size: 1.5rem;
    line-height: 1;
  }

  .posts {
    padding: 16px;
  }

  .post {
    display: grid;
    grid-template-columns: minmax(5rem, 1fr) repeat(3, 10rem);
  }

  .post:hover {
    background-color: rgb(0 0 0 / 0.1);
  }

  .post > .title {
    place-self: start;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .post > .name {
    display: block;
    width: 100%;
    overflow: hidden;
    text-align: end;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .post > .hash {
    place-self: start;
    color: var(--color-text-muted);
  }

  .post > .created-at {
    place-self: end;
  }

  .pagination {
    padding: 16px;
  }

  .pagination > ul {
    display: flex;
    flex-direction: row;
    justify-content: center;
    gap: 20px;
  }

  .pagination > ul > li > a {
    display: grid;
    place-items: center;
    outline: 1px solid var(--color-border);
    border-radius: 50%;
    background-color: var(--color-bg);
    width: 40px;
    height: 40px;
    color: var(--color-text-muted);
    font-weight: bold;
  }

  .pagination > ul > li > a.current {
    background-color: var(--color-primary);
    color: var(--color-bg);
  }
</style>
