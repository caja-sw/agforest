/**
 * @param {AuthorConstraint | undefined} constraint
 * @returns {string | undefined}
 */
const authorMessage = (constraint) => {
  if (constraint?.MIN_LENGTH_CONSTRAINT) {
    return `닉네임은 ${constraint.MIN_LENGTH_CONSTRAINT.min}글자보다 짧을 수 없습니다`;
  } else if (constraint?.MAX_LENGTH_CONSTRAINT) {
    return `닉네임은 ${constraint.MAX_LENGTH_CONSTRAINT.max}글자보다 길 수 없습니다`;
  } else if (constraint?.LINEBREAK_CONSTRAINT) {
    return `닉네임은 줄바꿈을 포함할 수 없습니다`;
  }
};

/**
 * @param {PasswordConstraint | undefined} constraint
 * @returns {string | undefined}
 */
const passwordMessage = (constraint) => {
  if (constraint?.PASSWORD_CONSTRAINT) {
    return "비밀번호는 공백과 DELETE를 제외한 ASCII 출력 가능 문자만 포함할 수 있습니다";
  } else if (constraint?.MIN_LENGTH_CONSTRAINT) {
    return `비밀번호는 ${constraint.MIN_LENGTH_CONSTRAINT.min}글자보다 짧을 수 없습니다`;
  } else if (constraint?.MAX_LENGTH_CONSTRAINT) {
    return `비밀번호는 ${constraint.MAX_LENGTH_CONSTRAINT.max}글자보다 길 수 없습니다`;
  }
};

/**
 * @param {PostTitleConstraint | undefined} constraint
 * @returns {string | undefined}
 */
const titleMessage = (constraint) => {
  if (constraint?.MIN_LENGTH_CONSTRAINT) {
    return `제목은 ${constraint.MIN_LENGTH_CONSTRAINT.min}글자보다 짧을 수 없습니다`;
  } else if (constraint?.MAX_LENGTH_CONSTRAINT) {
    return `제목은 ${constraint.MAX_LENGTH_CONSTRAINT.max}글자보다 길 수 없습니다`;
  }
};

/**
 * @param {PostContentConstraint | CommentContentConstraint | undefined} constraint
 * @returns {string | undefined}
 */
const contentMessage = (constraint) => {
  if (constraint?.MIN_LENGTH_CONSTRAINT) {
    return `내용은 ${constraint.MIN_LENGTH_CONSTRAINT.min}글자보다 짧을 수 없습니다`;
  } else if (constraint?.MAX_LENGTH_CONSTRAINT) {
    return `내용은 ${constraint.MAX_LENGTH_CONSTRAINT.max}글자보다 길 수 없습니다`;
  }
};

const UNKNOWN = "알 수 없는 오류가 발생했습니다";

/**
 * @param {CreatePostConstraints} param0
 */
export const getErrorMessagesFromCreatePostConstraints = ({ constraints }) => {
  const { author, password, title, content } = constraints;
  const messages = {};

  messages.author = authorMessage(author) ?? (author && UNKNOWN);
  messages.password = passwordMessage(password) ?? (password && UNKNOWN);
  messages.title = titleMessage(title) ?? (title && UNKNOWN);
  messages.content = contentMessage(content) ?? (content && UNKNOWN);

  return messages;
};

/**
 * @param {CreateCommentConstraints} param0
 */
export const getErrorMessagesFromCreateCommentConstraints = ({ constraints }) => {
  const { author, password, content } = constraints;
  const messages = {};

  messages.author = authorMessage(author) ?? (author && UNKNOWN);
  messages.password = passwordMessage(password) ?? (password && UNKNOWN);
  messages.content = contentMessage(content) ?? (content && UNKNOWN);

  return messages;
};
