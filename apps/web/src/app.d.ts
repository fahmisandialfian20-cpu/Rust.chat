declare global {
  namespace App {
    interface Error {
      code?: string;
      message: string;
    }

    interface Locals {
      accessToken?: string;
    }

    interface PageData {}

    interface PageState {}

    interface Platform {}
  }
}

export {};
