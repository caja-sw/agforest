<script>
  import "pretendard/dist/web/variable/pretendardvariable.css";
  import "./layout.css";

  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { backgroundImages } from "$lib/backgrounds";
  import { onMount } from "svelte";

  const { children } = $props();
  const { siteName, title, subtitle, description, canonical, article } = $derived(page.data);
  const fullTitle = $derived(`${!page.error ? title : page.status} — ${subtitle}`);

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
  <title>{fullTitle}</title>
  <meta name="description" content={description} />
  <meta name="color-scheme" content="light" />
  <meta name="robots" content={!page.error ? "index, follow" : "noindex, nofollow"} />
  <meta property="og:site_name" content={siteName} />
  <meta property="og:title" content={fullTitle} />
  <meta property="og:description" content={description} />
  {#if article}
    <meta property="og:type" content="article" />
    <meta property="article:published_time" content={article.publishedTime} />
    <meta property="article:section" content={article.section} />
  {:else}
    <meta property="og:type" content="website" />
  {/if}
  {#if !page.error}
    <meta property="og:url" content={canonical} />
    <link rel="canonical" href={canonical} />
  {/if}
  {#each backgroundImages as image (image)}
    <link rel="preload" as="image" href={image} />
  {/each}
</svelte:head>

<div class="min-h-screen">
  <div
    class="fixed inset-0 bg-(image:--bg-image) bg-cover bg-center bg-no-repeat opacity-60 duration-1000 ease-in-out"
    style:--bg-image={bgImage}
  ></div>
  <div class="fixed inset-0 bg-black/75"></div>
  <div class="mx-auto grid max-w-7xl md:gap-8 md:p-16">
    <header class="flex items-center justify-between p-4 md:p-0">
      <h1>
        <a href={resolve("/")}>
          <span class="site-title text-4xl font-bold md:text-6xl">
            {siteName}
          </span>
        </a>
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
