interface AuthorConstraint {
  MIN_LENGTH_CONSTRAINT?: { min: number };
  MAX_LENGTH_CONSTRAINT?: { max: number };
  LINEBREAK_CONSTRAINT?: {};
}

interface PasswordConstraint {
  MIN_LENGTH_CONSTRAINT?: { min: number };
  MAX_LENGTH_CONSTRAINT?: { max: number };
  PASSWORD_CONSTRAINT?: {};
}

interface PostTitleConstraint {
  MIN_LENGTH_CONSTRAINT?: { min: number };
  MAX_LENGTH_CONSTRAINT?: { max: number };
  LINEBREAK_CONSTRAINT?: {};
}

interface PostContentConstraint {
  MIN_LENGTH_CONSTRAINT?: { min: number };
  MAX_LENGTH_CONSTRAINT?: { max: number };
}

interface CommentContentConstraint {
  MIN_LENGTH_CONSTRAINT?: { min: number };
  MAX_LENGTH_CONSTRAINT?: { max: number };
}

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
    content?: CommentContentConstraint;
  };
}
