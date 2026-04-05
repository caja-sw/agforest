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
  comments: CommentListItem[];
}
