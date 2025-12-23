<script>
  const id = $props.id();
  /** @type {{ label: string, values: { id: number, name: string }[], value: { id: number, name: string } }} */
  let { label, values, value = $bindable() } = $props();
</script>

<div>
  <label class="text-sm font-semibold" for={id}>{label}</label>
  <select
    class="card focus:outline-primary w-full appearance-none p-3"
    {id}
    bind:value
  >
    {#each values as value (value.id)}
      <option {value}>{value.name}</option>
    {/each}
  </select>
</div>
