<script>
  const id = $props.id();
  /** @type {{ label: string, type: "text" | "password", value: string, error: string }} */
  let { label, type, value = $bindable(), error = $bindable() } = $props();
</script>

<div>
  <label class="text-sm font-semibold" for={id}>{label}</label>
  <input
    class={[
      "card focus:outline-primary w-full p-3",
      error.length > 0 && "outline-accent",
    ]}
    {id}
    {type}
    bind:value
    class:error={error.length > 0}
    oninput={() => (error = "")}
  />
  <span class="text-accent">{error}</span>
</div>
