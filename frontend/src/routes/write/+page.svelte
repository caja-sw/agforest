<script>
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { createPost } from "$lib/api";
  import ContentField from "$lib/components/ContentField.svelte";
  import InputField from "$lib/components/InputField.svelte";
  import SelectField from "$lib/components/SelectField.svelte";
  import SubmitButton from "$lib/components/SubmitButton.svelte";

  const { data } = $props();
  const { categories } = $derived(data);

  // svelte-ignore state_referenced_locally
  let category = $state(categories[0]);
  let author = $state("");
  let password = $state("");
  let title = $state("");
  let content = $state("");
  let authorError = $state("");
  let passwordError = $state("");
  let titleError = $state("");
  let contentError = $state("");

  /** @param {SubmitEvent} event */
  async function submit(event) {
    event.preventDefault();

    try {
      const categoryId = category?.id;
      if (categoryId === undefined) return;
      const { id } = await createPost({ categoryId, author, password, title, content });
      goto(resolve("/post/[id]", { id: String(id) }));
    } catch (errRes) {
      if (!(errRes instanceof Response)) throw errRes;

      switch (errRes.status) {
        case 404: {
          alert("카테고리가 존재하지 않습니다");
          break;
        }
        case 422: {
          /** @type {{ messages: { author?: string, password?: string, title?: string, content?: string } }} */
          const { messages } = await errRes.json();
          authorError = messages.author ?? "";
          passwordError = messages.password ?? "";
          titleError = messages.title ?? "";
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

<div class="container glass">
  <h1 class="header">게시글 작성</h1>
  <form onsubmit={submit} novalidate>
    <div class="meta">
      <div class="category">
        <SelectField label="카테고리" values={categories} bind:value={category} />
      </div>
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
      <div class="title">
        <InputField label="제목" type="text" bind:value={title} bind:error={titleError} />
      </div>
    </div>

    <ContentField bind:value={content} bind:error={contentError} minLines={10} />

    <div class="actions">
      <SubmitButton />
    </div>
  </form>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 24px;
    width: 100%;
  }

  .header {
    display: block;
    width: max-content;
    font-size: 1.5rem;
    line-height: 1;
  }

  form {
    display: grid;
    gap: 16px;
  }

  .meta {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-template-areas:
      "category .       "
      "author   password"
      "title    title   ";
    gap: 4px 16px;
  }

  .category {
    grid-area: category;
  }

  .author {
    grid-area: author;
  }

  .password {
    grid-area: password;
  }

  .title {
    grid-area: title;
  }

  .actions {
    place-self: end;
  }
</style>
