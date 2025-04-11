export interface AppError {
  kind: string;
  message: string;
}

export function isAppError(error: unknown): error is AppError {
  return (
    typeof error === 'object'
    && error !== null
    && 'kind' in error
    && 'message' in error
    && typeof (error as Record<string, unknown>).kind === 'string'
    && typeof (error as Record<string, unknown>).message === 'string'
  );
}

export function toAppError(maybeError: unknown): AppError {
  if (isAppError(maybeError)) {
    return maybeError;
  }

  try {
    return {
      kind: 'unprocessable',
      message: JSON.stringify(maybeError),
    };
  }
  catch {
    return {
      kind: 'unprocessable',
      message: String(maybeError),
    };
  }
}
