import type { ClassValue } from 'clsx';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import { locale } from './translations/translations';

export const ULID_NIL: string = '00000000000000000000000000';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export type WithoutChild<T> = T extends { child?: any } ? Omit<T, 'child'> : T;
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, 'children'> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };

export function* range(start: number, end: number) {
  for (let i = start; i < end; i++) {
    yield i;
  }
}

enum DateFormat {
  Ymd,
  PlainYmd,
  YmdAndDayOfWeek,
}

namespace DateFormat {
  export function format(format: DateFormat, _date: Date): string {
    const date = new Date(_date);
    switch (format) {
      case DateFormat.Ymd: {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        if (locale.get() === 'ja') {
          return `${year}年${month}月${day}日`;
        }
        else {
          return `${year}-${month}-${day}`;
        }
      }
      case DateFormat.PlainYmd:
        return `${date.getFullYear()}/${String(date.getMonth() + 1).padStart(2, '0')}/${String(date.getDate()).padStart(2, '0')}`;
      case DateFormat.YmdAndDayOfWeek:{
        let dayOfWeek: string;
        if (locale.get() === 'ja') {
          const daysOfWeek: string[] = ['日', '月', '火', '水', '木', '金', '土'];
          dayOfWeek = daysOfWeek[date.getDay()];
        }
        else {
          const daysOfWeek: string[] = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
          dayOfWeek = daysOfWeek[date.getDay()];
        }
        return `${DateFormat.format(DateFormat.Ymd, date)} (${dayOfWeek})`;
      }
    }
  }
}

enum TimeFormat {
  PlainHm,
  PlainHms,
}

namespace TimeFormat {
  export function format_secs(format: TimeFormat, time: number): string {
    const hours = Math.floor(time / 3600);
    const minutes = Math.floor((time % 3600) / 60);
    const seconds = Math.floor(time % 60);

    let text;
    if (hours > 0) {
      text = `${hours}:`;
    }
    else {
      text = '';
    }

    switch (format) {
      case TimeFormat.PlainHm:
        return `${text}${String(minutes).padStart(2, '0')}`;
      case TimeFormat.PlainHms:
        return `${text}${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
    }
  }

  export function format(format: TimeFormat, _time: Date): string {
    const time = new Date(_time);
    const hours = String(time.getHours()).padStart(2, '0');
    const minutes = String(time.getMinutes()).padStart(2, '0');
    const seconds = String(time.getSeconds()).padStart(2, '0');

    switch (format) {
      case TimeFormat.PlainHm:
        return `${hours}:${minutes}`;
      case TimeFormat.PlainHms:
        return `${hours}:${minutes}:${seconds}`;
    }
  }
}

enum DateTimeFormat {
  YmdHm,
  PlainYmdHm,
  PlainYmdHms,
  YmdHmAndDayOfWeek,
}

namespace DateTimeFormat {
  export function formatChoice(datetime: Date, date: DateFormat, time: TimeFormat): string {
    const dateText = DateFormat.format(date, datetime);
    const timeText = TimeFormat.format(time, datetime);
    return `${dateText} ${timeText}`;
  }

  export function format(format: DateTimeFormat, _datetime: Date): string {
    const datetime = new Date(_datetime);
    switch (format) {
      case DateTimeFormat.YmdHm:
        return DateTimeFormat.formatChoice(datetime, DateFormat.Ymd, TimeFormat.PlainHm);
      case DateTimeFormat.PlainYmdHm:
        return DateTimeFormat.formatChoice(datetime, DateFormat.PlainYmd, TimeFormat.PlainHm);
      case DateTimeFormat.PlainYmdHms:
        return DateTimeFormat.formatChoice(datetime, DateFormat.PlainYmd, TimeFormat.PlainHms);
      case DateTimeFormat.YmdHmAndDayOfWeek:
        return DateTimeFormat.formatChoice(datetime, DateFormat.YmdAndDayOfWeek, TimeFormat.PlainHm);
    }
  }
}

export { DateFormat, DateTimeFormat, TimeFormat };
