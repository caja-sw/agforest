<script>
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { createPost } from "$lib/api";
  import { ContentField, InputField, SelectField, SubmitButton } from "$lib/components";
  import { getErrorMessagesFromCreatePostConstraints } from "$lib/helper";

  const { data } = $props();

  let category = $derived(data.category);
  let author = $state("");
  let password = $state("");
  let title = $state("");
  let content = $state("");
  let authorError = $state("");
  let passwordError = $state("");
  let titleError = $state("");
  let contentError = $state("");

  let uploading = $state(false);

  /** @param {SubmitEvent} event */
  async function submit(event) {
    event.preventDefault();

    if (uploading) {
      return;
    }
    uploading = true;

    try {
      const { id } = await createPost({
        categoryId: category.id,
        author,
        password,
        title,
        content,
      });
      goto(resolve("/post/[id=id]", { id: String(id) }));
    } catch (errRes) {
      if (!(errRes instanceof Response)) {
        throw errRes;
      }

      if (errRes.status === 404) {
        alert("카테고리가 존재하지 않습니다");
      } else if (errRes.status === 422) {
        const error = await errRes.json();
        const messages = getErrorMessagesFromCreatePostConstraints(error);
        authorError = messages.author ?? "";
        passwordError = messages.password ?? "";
        titleError = messages.title ?? "";
        contentError = messages.content ?? "";
      } else {
        alert("알 수 없는 오류가 발생했습니다");
      }
    } finally {
      uploading = false;
    }
  }
</script>

<div class="glass flex w-full flex-col gap-4 p-6">
  <h1 class="block w-max text-2xl leading-none">게시글 작성</h1>
  <form class="grid gap-4" onsubmit={submit} novalidate>
    <div class="grid grid-cols-2 gap-x-4 gap-y-1">
      <div class="row-start-1">
        <SelectField label="카테고리" values={data.categories} bind:value={category} />
      </div>
      <div class="row-start-2">
        <InputField label="닉네임" type="text" bind:value={author} bind:error={authorError} />
      </div>
      <div class="row-start-2">
        <InputField
          label="비밀번호"
          type="password"
          bind:value={password}
          bind:error={passwordError}
        />
      </div>
      <div class="col-span-full row-start-3">
        <InputField label="제목" type="text" bind:value={title} bind:error={titleError} />
      </div>
    </div>

    <ContentField bind:value={content} bind:error={contentError} minLines={10} />

    <div class="place-self-end">
      <SubmitButton disabled={uploading} />
    </div>
  </form>
</div>
