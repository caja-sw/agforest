interface PostListItem {
  id: number;
  author: {
    name: string;
    hash: string;
  };
  title: string;
  createdAt: string;
  commentCount: number;
}

interface Post {
  id: number;
  category: {
    id: number;
    name: string;
  };
  author: {
    name: string;
    hash: string;
  };
  title: string;
  content: string;
  createdAt: string;
  comments: CommentListItem[];
}
