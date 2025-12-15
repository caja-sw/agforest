<script>
  import { createComment } from "$lib/api";
  import ContentField from "$lib/components/ContentField.svelte";
  import InputField from "$lib/components/InputField.svelte";
  import SubmitButton from "$lib/components/SubmitButton.svelte";

  /** @type {{ postId: number, onupdate: () => void }} */
  const { postId, onupdate } = $props();

  let author = $state("");
  let password = $state("");
  let content = $state("");
  let authorError = $state("");
  let passwordError = $state("");
  let contentError = $state("");

  /** @param {SubmitEvent} event */
  async function submit(event) {
    event.preventDefault();

    try {
      await createComment({ postId, author, password, content });
      content = "";
      onupdate();
    } catch (errRes) {
      if (!(errRes instanceof Response)) throw errRes;

      switch (errRes.status) {
        case 404: {
          alert("카테고리가 존재하지 않습니다");
          break;
        }
        case 422: {
          /** @type {{ messages: { author?: string, password?: string, content?: string } }} */
          const { messages } = await errRes.json();
          authorError = messages.author ?? "";
          passwordError = messages.password ?? "";
          contentError = messages.content ?? "";
          break;
        }
        default: {
          alert("알 수 없는 오류가 발생했습니다");
        }
      }
    }
  }
</script>

<section class="container glass">
  <h1 class="header">댓글 작성</h1>
  <form onsubmit={submit} novalidate>
    <div class="meta">
      <div class="author">
        <InputField label="닉네임" type="text" bind:value={author} bind:error={authorError} />
      </div>
      <div class="password">
        <InputField
          label="비밀번호"
          type="password"
          bind:value={password}
          bind:error={passwordError}
        />
      </div>
    </div>

    <ContentField bind:value={content} bind:error={contentError} minLines={3} />

    <div class="actions">
      <SubmitButton />
    </div>
  </form>
</section>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 24px;
  }

  .header {
    font-size: 1.2rem;
  }

  form {
    display: grid;
    gap: 16px;
  }

  .meta {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4px 16px;
  }

  .actions {
    place-self: end;
  }
</style>
