<script>
  /** @type {import("svelte/elements").HTMLButtonAttributes} */
  const props = $props();
</script>

<button
  class="bg-accent disabled:bg-accent/50 text-bg rounded-xl px-6 py-3 font-bold hover:cursor-pointer"
  {...props}>삭제</button
>
