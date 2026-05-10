<script>
  /** @type {import("svelte/elements").HTMLButtonAttributes & { liked: boolean; count: number }} */
  const { liked, count, ...props } = $props();
</script>

<button
  class={[
    "border-like hover:bg-like hover:text-base-100 rounded-xl border transition-all duration-200 ease-in-out hover:scale-105 hover:cursor-pointer active:scale-95",
    liked && "disabled:bg-like/50 bg-like text-base-100",
    !liked && "disabled:bg-base-100/50 bg-base-100 text-like",
  ]}
  {...props}
>
  <div class="px-6 py-3 font-bold">
    <span>♡ {count}</span>
  </div>
</button>
