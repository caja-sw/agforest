<script>
  /** @type {{ value: string, error: string, minLines: number }} */
  let { value = $bindable(), error = $bindable(), minLines } = $props();
</script>

<div>
  <textarea
    class="card"
    name="content"
    bind:value
    style:--min-lines={minLines}
    class:error={error.length > 0}
    oninput={({ currentTarget }) => {
      error = "";
      currentTarget.style.height = "auto";
      currentTarget.style.height = `${currentTarget.scrollHeight}px`;
    }}
  ></textarea>
  <span>{error}</span>
</div>

<style>
  textarea {
    padding: 10px;
    width: 100%;
    min-height: calc(1lh * var(--min-lines) + 20px);
    resize: none;
  }

  textarea:focus {
    outline-color: var(--color-primary);
  }

  textarea.error {
    outline-color: var(--color-accent);
  }

  span {
    color: var(--color-accent);
  }
</style>
