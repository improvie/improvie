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
  static log(level: keyof typeof LogLevel, message: string): void {
    const logFunction = LogLevel[level];
    logFunction(message);
  }

  static trace(message: string): void {
    this.log('trace', message);
  }

  static debug(message: string): void {
    this.log('debug', message);
  }

  static info(message: string): void {
    this.log('info', message);
  }

  static warn(message: string): void {
    this.log('warn', message);
  }

  static error(message: string): void {
    this.log('error', message);
  }

  static app_error(message: string, app_error: AppError): void {
    this.error(`${message} - ${app_error.kind}: ${app_error.message}`);
  }
}
