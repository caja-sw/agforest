<script>
  /** @type {{ value: string, error: string, minLines: number }} */
  let { value = $bindable(), error = $bindable(), minLines } = $props();
</script>

<div>
  <textarea
    class={[
      "card focus:outline-primary min-h-[calc(1lh*var(--min-lines)+20px)] w-full resize-none p-3",
      error.length > 0 && "outline-accent",
    ]}
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
  <span class="text-accent">{error}</span>
</div>
