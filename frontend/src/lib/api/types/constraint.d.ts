interface MinLengthConstraint {
  type: "MIN_LENGTH_CONSTRAINT";
  value: number;
}
interface MaxLengthConstraint {
  type: "MAX_LENGTH_CONSTRAINT";
  value: number;
}
interface LinebreakConstraint {
  type: "LINEBREAK_CONSTRAINT";
}
interface PasswordConstraint {
  type: "PASSWORD_CONSTRAINT";
}

type AuthorConstraint =
  | MinLengthConstraint
  | MaxLengthConstraint
  | LinebreakConstraint;

type PasswordConstraint =
  | PasswordConstraint
  | MinLengthConstraint
  | MaxLengthConstraint;

type PostTitleConstraint =
  | MinLengthConstraint
  | MaxLengthConstraint
  | LinebreakConstraint;

type PostContentConstraint = MinLengthConstraint | MaxLengthConstraint;

type CommentContentConstraint = MinLengthConstraint | MaxLengthConstraint;

interface CreatePostConstraints {
  constraints: {
    author?: AuthorConstraint;
    password?: PasswordConstraint;
    title?: PostTitleConstraint;
    content?: PostContentConstraint;
  };
}

interface CreateCommentConstraints {
  constraints: {
    author?: AuthorConstraint;
    password?: PasswordConstraint;
    content?: PostContentConstraint;
  };
}
