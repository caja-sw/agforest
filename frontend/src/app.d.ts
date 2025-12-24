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
}

export {};
