import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export const UUID_NIL: string = "00000000-0000-0000-0000-000000000000";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function* range(start: number, end: number) {
  for (let i = start; i < end; i++) {
    yield i;
  }
}
