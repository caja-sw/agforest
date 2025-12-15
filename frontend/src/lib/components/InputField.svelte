<script>
  const id = $props.id();
  /** @type {{ label: string, type: "text" | "password", value: string, error: string }} */
  let { label, type, value = $bindable(), error = $bindable() } = $props();
</script>

<div>
  <label for={id}>{label}</label>
  <input
    class="card"
    {id}
    {type}
    bind:value
    class:error={error.length > 0}
    oninput={() => (error = "")}
  />
  <span>{error}</span>
</div>

<style>
  label {
    display: inline-block;
    width: max-content;
    font-weight: 600;
    font-size: 0.8rem;
  }

  input {
    padding: 10px;
    width: 100%;
  }

  input:focus {
    outline-color: var(--color-primary);
  }

  input.error {
    outline-color: var(--color-accent);
  }

  span {
    color: var(--color-accent);
  }
</style>
