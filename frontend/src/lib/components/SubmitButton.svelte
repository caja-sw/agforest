<script>
  /** @type {Omit<import("svelte/elements").HTMLButtonAttributes, "type">} */
  const props = $props();
</script>

<button
  class="bg-primary disabled:bg-primary/50 text-bg rounded-xl px-6 py-3 font-bold hover:cursor-pointer"
  type="submit"
  {...props}>등록</button
>
