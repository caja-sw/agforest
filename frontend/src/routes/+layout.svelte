<script>
  import "./layout.css";

  import { asset, resolve } from "$app/paths";
  import { page } from "$app/state";
  import { onMount } from "svelte";

  const { children } = $props();
  const { title, description, ogtype } = page.data;
  const canonicalHref = new URL(page.url.pathname, page.url.origin).href;

  const images = [
    asset("/images/school-01.jpg"),
    asset("/images/school-02.jpg"),
    asset("/images/school-03.jpg"),
    asset("/images/school-04.jpg"),
    asset("/images/school-05.jpg"),
    asset("/images/school-06.jpg"),
    asset("/images/school-07.jpg"),
    asset("/images/school-08.jpg"),
    asset("/images/school-09.jpg"),
    asset("/images/school-10.jpg"),
    asset("/images/school-11.jpg"),
    asset("/images/school-12.jpg"),
  ];

  let bgImage = $state("");

  onMount(() => {
    let index = 0;
    function setImage() {
      bgImage = `url(${images[index]})`;
      index = (index + 1) % images.length;
      const nextSrc = images[index];
      const img = new Image();
      img.src = nextSrc;
    }
    setImage();
    const id = setInterval(setImage, 5000);
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
  <meta property="og:type" content={ogtype} />
</svelte:head>

<div
  class="min-h-screen before:fixed before:inset-0 before:-z-10 before:bg-(--bg-image) before:bg-cover before:bg-center before:bg-no-repeat before:opacity-60 before:duration-1000 before:ease-in-out before:content-['']"
  style:--bg-image={bgImage}
>
  <div class="mx-auto grid w-full max-w-7xl min-w-min gap-8 p-16">
    <header>
      <h1>
        <a
          class="inline-block w-max text-6xl leading-none font-semibold text-bg text-shadow-lg/30"
          href={resolve("/")}>앙고나무숲</a
        >
      </h1>
    </header>
    <main class="w-full">{@render children()}</main>
  </div>
</div>
