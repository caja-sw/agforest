/** @param {AuthorConstraint | undefined} constraint */
function authorMessage(constraint) {
  switch (constraint?.type) {
    case "MIN_LENGTH_CONSTRAINT":
      return `닉네임은 ${constraint.value}글자보다 짧을 수 없습니다`;
    case "MAX_LENGTH_CONSTRAINT":
      return `닉네임은 ${constraint.value}글자보다 길 수 없습니다`;
    case "LINEBREAK_CONSTRAINT":
      return `닉네임은 줄바꿈을 포함할 수 없습니다`;
  }
}

/** @param {PasswordConstraint | undefined} constraint */
function passwordMessage(constraint) {
  switch (constraint?.type) {
    case "CHARACTER_CONSTRAINT":
      return "비밀번호는 공백과 DELETE를 제외한 ASCII 출력 가능 문자만 포함할 수 있습니다";
    case "MIN_LENGTH_CONSTRAINT":
      return `비밀번호는 ${constraint.value}글자보다 짧을 수 없습니다`;
    case "MAX_LENGTH_CONSTRAINT":
      return `비밀번호는 ${constraint.value}글자보다 길 수 없습니다`;
  }
}

/** @param {PostTitleConstraint | undefined} constraint */
function titleMessage(constraint) {
  switch (constraint?.type) {
    case "MIN_LENGTH_CONSTRAINT":
      return `제목은 ${constraint.value}글자보다 짧을 수 없습니다`;
    case "MAX_LENGTH_CONSTRAINT":
      return `제목은 ${constraint.value}글자보다 길 수 없습니다`;
    case "LINEBREAK_CONSTRAINT":
      return `제목은 줄바꿈을 포함할 수 없습니다`;
  }
}

/** @param {PostContentConstraint | CommentContentConstraint | undefined} constraint */
function contentMessage(constraint) {
  switch (constraint?.type) {
    case "MIN_LENGTH_CONSTRAINT":
      return `내용은 ${constraint.value}글자보다 짧을 수 없습니다`;
    case "MAX_LENGTH_CONSTRAINT":
      return `내용은 ${constraint.value}글자보다 길 수 없습니다`;
  }
}

const UNKNOWN = "알 수 없는 오류가 발생했습니다";

/**
 * @param {CreatePostConstraints} param0
 */
export function getErrorMessagesFromCreatePostConstraints({ constraints }) {
  const { author, password, title, content } = constraints;
  const messages = {};

  messages.author = authorMessage(author) ?? (author && UNKNOWN);
  messages.password ??= passwordMessage(password) ?? (password && UNKNOWN);
  messages.title ??= titleMessage(title) ?? (title && UNKNOWN);
  messages.content ??= contentMessage(content) ?? (content && UNKNOWN);

  return messages;
}

/**
 * @param {CreateCommentConstraints} param0
 */
export function getErrorMessagesFromCreateCommentConstraints({ constraints }) {
  const { author, password, content } = constraints;
  const messages = {};

  messages.author = authorMessage(author) ?? (author && UNKNOWN);
  messages.password ??= passwordMessage(password) ?? (password && UNKNOWN);
  messages.content ??= contentMessage(content) ?? (content && UNKNOWN);

  return messages;
}
