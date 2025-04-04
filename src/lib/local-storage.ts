export function getLocalStorage(key: string): string | null {
  if (typeof window !== 'undefined') {
    return localStorage.getItem(key);
  }
  return null;
}

export function setLocalStorage(key: string, value: string): void {
  if (typeof window !== 'undefined') {
    localStorage.setItem(key, value);
  }
}

export function removeLocalStorage(key: string): void {
  if (typeof window !== 'undefined') {
    localStorage.removeItem(key);
  }
}

export function getLocalStorageOrDefault(key: string, defaultValue: string): string {
  const value = getLocalStorage(key);
  if (value !== null) {
    return value;
  }
  setLocalStorage(key, defaultValue);
  return defaultValue;
}
