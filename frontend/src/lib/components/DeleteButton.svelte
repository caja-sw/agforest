<script>
  /** @type {import("svelte/elements").HTMLButtonAttributes} */
  const props = $props();
</script>

<button class="bg-warn disabled:bg-warn/50 rounded-xl px-6 py-3 hover:cursor-pointer" {...props}>
  <span class="text-bg font-bold">삭제</span>
</button>
