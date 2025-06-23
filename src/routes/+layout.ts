// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)

import { debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

/* eslint-disable no-console */
function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (...data: any[]) => void,
) {
  const original = console[fnName];
  console[fnName] = (...data: any[]) => {
    original(...data);
    const message = data.map(d => (typeof d === 'object' ? JSON.stringify(d) : d)).join(' ');
    logger(message);
  };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);
