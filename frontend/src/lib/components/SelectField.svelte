<script>
  const id = $props.id();
  /** @type {{ label: string, values: { id: number, name: string }[], value: { id: number, name: string } }} */
  let { label, values, value = $bindable() } = $props();
</script>

<div>
  <label for={id}>{label}</label>
  <select class="card" {id} bind:value>
    {#each values as value (value.id)}
      <option {value}>{value.name}</option>
    {/each}
  </select>
</div>

<style>
  label {
    display: inline-block;
    width: max-content;
    font-weight: 600;
    font-size: 0.8rem;
  }

  select {
    appearance: none;
    padding: 10px;
    width: 100%;
  }

  select:focus {
    outline-color: var(--color-primary);
  }
</style>
