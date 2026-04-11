interface CommentListItem {
  id: number;
  author: {
    name: string;
    hash: string;
  };
  content: string;
  likeCount: number;
  createdAt: string;
}
