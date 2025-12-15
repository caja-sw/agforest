<script>
  import "tailwindcss/preflight.css";
  import "../app.css";

  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { onMount } from "svelte";

  const { children } = $props();

  const images = [
    "/school-01.jpg",
    "/school-02.jpg",
    "/school-03.jpg",
    "/school-04.jpg",
    "/school-05.jpg",
    "/school-06.jpg",
    "/school-07.jpg",
    "/school-08.jpg",
    "/school-09.jpg",
    "/school-10.jpg",
    "/school-11.jpg",
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
  <title>{page.data.title} — 앙고나무숲</title>
</svelte:head>

<div class="container" style:--bg-image={bgImage}>
  <div class="content">
    <header><h1><a class="title" href={resolve("/")}>앙고나무숲</a></h1></header>
    <main>{@render children()}</main>
  </div>
</div>

<style>
  .container {
    min-height: 100vh;
  }

  .container::before {
    position: fixed;
    opacity: 0.6;
    z-index: -1;
    transition: background-image 1s ease-in-out;
    inset: 0;
    background: var(--bg-image) center / cover no-repeat;
    content: "";
  }

  .content {
    display: grid;
    gap: 32px;
    margin: 0 auto;
    padding: 64px;
    width: 100%;
    min-width: min-content;
    max-width: 75rem;
  }

  .title {
    display: inline-block;
    width: max-content;
    color: var(--color-bg);
    font-weight: 600;
    font-size: 4rem;
    line-height: 1;
    text-shadow: 2px 2px 5px rgb(0 0 0 / 0.4);
  }

  main {
    width: 100%;
  }
</style>
