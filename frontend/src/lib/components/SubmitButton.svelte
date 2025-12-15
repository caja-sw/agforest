<script>
  /** @type {Omit<import("svelte/elements").HTMLButtonAttributes, "type">} */
  const props = $props();
</script>

<button type="submit" {...props}>등록</button>

<style>
  button {
    border-radius: var(--radius);
    background-color: var(--color-primary);
    padding: 10px 20px;
    color: var(--color-bg);
    font-weight: 700;
  }

  button:hover {
    cursor: pointer;
  }
</style>
