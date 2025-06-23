import type { AppError } from './error';
import { debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

export const LogLevel = {
  trace,
  debug,
  info,
  warn,
  error,
};

export class Logger {
  static format(...data: any[]): string {
    return data.map(d => (typeof d === 'object' ? JSON.stringify(d) : d)).join(' ');
  }

  static log(level: keyof typeof LogLevel, message: string): void {
    const logFunction = LogLevel[level];
    logFunction(message);
  }

  static trace(...data: any[]): void {
    this.log('trace', this.format(...data));
  }

  static debug(...data: any[]): void {
    this.log('debug', this.format(...data));
  }

  static info(...data: any[]): void {
    this.log('info', this.format(...data));
  }

  static warn(...data: any[]): void {
    this.log('warn', this.format(...data));
  }

  static error(...data: any[]): void {
    this.log('error', this.format(...data));
  }

  static app_error(app_error: AppError, ...data: any[]): void {
    const message = this.format(...data);
    this.error(`${message} - ${app_error.kind}: ${app_error.message}`);
  }
}
