<script>
  /** @type {import("svelte/elements").HTMLButtonAttributes} */
  const props = $props();
</script>

<button {...props}>삭제</button>

<style>
  button {
    border-radius: var(--radius);
    background-color: var(--color-accent);
    padding: 10px 20px;
    color: var(--color-bg);
    font-weight: 700;
  }

  button:hover {
    cursor: pointer;
  }
</style>
