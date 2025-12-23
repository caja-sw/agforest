type MinLengthConstraint = {
  type: "MIN_LENGTH_CONSTRAINT";
  value: number;
};
type MaxLengthConstraint = {
  type: "MAX_LENGTH_CONSTRAINT";
  value: number;
};
type LinebreakConstraint = {
  type: "LINEBREAK_CONSTRAINT";
};
type CharacterConstraint = {
  type: "CHARACTER_CONSTRAINT";
};

type AuthorConstraint =
  | MinLengthConstraint
  | MaxLengthConstraint
  | LinebreakConstraint;

type PasswordConstraint =
  | CharacterConstraint
  | MinLengthConstraint
  | MaxLengthConstraint;

type PostTitleConstraint =
  | MinLengthConstraint
  | MaxLengthConstraint
  | LinebreakConstraint;

type PostContentConstraint = MinLengthConstraint | MaxLengthConstraint;

type CommentContentConstraint = MinLengthConstraint | MaxLengthConstraint;

declare global {
  namespace App {
    interface PageData {
      title: string;
      description?: string;
      article?: {
        publishedTime: string;
        section: string;
      };
    }
  }

  type CreatePostConstraints = {
    constraints: {
      author?: AuthorConstraint;
      password?: PasswordConstraint;
      title?: PostTitleConstraint;
      content?: PostContentConstraint;
    };
  };

  type CreateCommentConstraints = {
    constraints: {
      author?: AuthorConstraint;
      password?: PasswordConstraint;
      content?: PostContentConstraint;
    };
  };
}

export {};
