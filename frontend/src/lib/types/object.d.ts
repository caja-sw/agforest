interface CategoryListItem {
  id: number;
  name: string;
}

interface Category {
  id: number;
  name: string;
  readonly: boolean;
  postCount: number;
}

interface PostListItem {
  id: number;
  author: {
    name: string;
    hash: string;
  };
  title: string;
  likeCount: number;
  createdAt: string;
  commentCount: number;
}

interface Post {
  id: number;
  category: {
    id: number;
    name: string;
    readonly: boolean;
  };
  author: {
    name: string;
    hash: string;
  };
  title: string;
  content: string;
  likeCount: number;
  createdAt: string;
  comments: Comment[];
}

interface Comment {
  id: number;
  author: {
    name: string;
    hash: string;
  };
  content: string;
  likeCount: number;
  createdAt: string;
}
