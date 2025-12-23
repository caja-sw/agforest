<script>
  import { invalidateAll } from "$app/navigation";
  import { createComment } from "$lib/api";
  import { ContentField, InputField, SubmitButton } from "$lib/components";
  import { getErrorMessagesFromCreateCommentConstraints } from "$lib/helper/get-error-messages";

  /** @type {{ postId: number }} */
  const { postId } = $props();

  let author = $state("");
  let password = $state("");
  let content = $state("");
  let authorError = $state("");
  let passwordError = $state("");
  let contentError = $state("");

  let uploading = $state(false);

  /** @param {SubmitEvent} event */
  async function submit(event) {
    event.preventDefault();

    uploading = true;
    try {
      await createComment({ postId, author, password, content });
      content = "";
      invalidateAll();
    } catch (errRes) {
      if (!(errRes instanceof Response)) throw errRes;

      switch (errRes.status) {
        case 404:
          alert("카테고리가 존재하지 않습니다");
          break;
        case 422: {
          /** @type {CreateCommentConstraints} */
          const error = await errRes.json();
          const message = getErrorMessagesFromCreateCommentConstraints(error);
          authorError = message.author ?? "";
          passwordError = message.password ?? "";
          contentError = message.content ?? "";
          break;
        }
        default:
          alert("알 수 없는 오류가 발생했습니다");
      }
    } finally {
      uploading = false;
    }
  }
</script>

<section class="glass grid gap-2 p-6">
  <h1 class="text-xl">댓글 작성</h1>
  <form class="grid gap-4" onsubmit={submit} novalidate>
    <div class="grid grid-cols-2 gap-x-4 gap-y-1">
      <InputField
        label="닉네임"
        type="text"
        bind:value={author}
        bind:error={authorError}
      />
      <InputField
        label="비밀번호"
        type="password"
        bind:value={password}
        bind:error={passwordError}
      />
    </div>

    <ContentField bind:value={content} bind:error={contentError} minLines={3} />

    <div class="place-self-end">
      <SubmitButton disabled={uploading} />
    </div>
  </form>
</section>
