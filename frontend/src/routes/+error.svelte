<script>
  import { page } from "$app/state";
</script>

<div class="glass p-4">
  <h1>{page.error?.message}</h1>
</div>
