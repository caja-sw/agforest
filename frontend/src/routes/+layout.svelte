<script>
  import "./layout.css";

  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { backgroundImages } from "$lib/backgrounds";
  import { onMount } from "svelte";

  const { children } = $props();
  const {
    title,
    description = "앙고나무숲: 천안중앙고등학교 익명 커뮤니티. SW융합부 개발",
    article,
  } = page.data;
  const canonicalHref = new URL(page.url.pathname, page.url.origin).href;

  let bgImageUrl = $state(backgroundImages[0]);
  const bgImage = $derived(`url(${bgImageUrl})`);

  onMount(() => {
    let index = 1;
    const id = setInterval(() => {
      bgImageUrl = backgroundImages[index];
      index = (index + 1) % backgroundImages.length;
    }, 5000);
    return () => clearInterval(id);
  });
</script>

<svelte:head>
  <title>{title} — 앙고나무숲</title>
  <meta name="description" content={description} />
  <meta name="color-scheme" content="light" />
  <meta name="robots" content="index, follow" />
  <meta property="og:site_name" content="앙고나무숲" />
  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={canonicalHref} />
  {#if article}
    <meta property="og:type" content="article" />
    <meta property="article:published_time" content={article.publishedTime} />
    <meta property="article:section" content={article.section} />
  {:else}
    <meta property="og:type" content="website" />
  {/if}
  {#each backgroundImages as image (image)}
    <link rel="preload" as="image" href={image} />
  {/each}
</svelte:head>

<div
  class="min-h-screen before:fixed before:inset-0 before:-z-10 before:bg-(image:--bg-image) before:bg-cover before:bg-center before:bg-no-repeat before:opacity-60 before:duration-1000 before:ease-in-out before:content-['']"
  style:--bg-image={bgImage}
>
  <div class="mx-auto grid max-w-7xl md:gap-8 md:p-16">
    <header class="flex items-center justify-between p-4 md:p-0">
      <h1>
        <a
          class="text-bg text-4xl font-semibold text-shadow-lg/20 md:text-6xl"
          href={resolve("/")}>앙고나무숲</a
        >
      </h1>
      <a href="https://github.com/caja-sw/agforest" target="_blank">
        <div class="glass text-text-muted hover:text-text p-2">
          <span>GitHub</span>
        </div>
      </a>
    </header>
    <main>{@render children()}</main>
  </div>
</div>
