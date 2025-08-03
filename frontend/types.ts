export enum Screen {
  Login,
  Exchange,
  Confirmation,
}

export interface User {
  email: string;
  name: string;
  benyPoints: number;
}

export interface Token {
  name: string;
  code: string;
  balance: number;
}