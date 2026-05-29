declare global {
  namespace App {
    interface PageData {
      siteName: string;
      title: string;
      subtitle: string;
      description: string;
      canonical: string;
      article?: {
        publishedTime: string;
        section: string;
      };
    }
  }
}

export {};
