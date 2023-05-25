import * as O from './option';

class Result<const T, const E> {
  constructor(
    private inner: { kind: 'ok'; value: T } | { kind: 'err'; reason: E }
  ) {}

  eq(other: Result<T, E>): boolean {
    if (this.inner.kind === 'ok') {
      if (other.inner.kind === 'err') return false;
      return this.inner.value === other.inner.value;
    }

    if (other.inner.kind === 'ok') return false;
    return this.inner.reason === other.inner.reason;
  }

  ok(): O.Option<T> {
    return this.inner.kind === 'ok' ? O.some(this.inner.value) : O.none();
  }
}

class ResultComputation<const T, const E> {
  constructor(private currentResult: Result<T, E>) {}
}

export type { Result };

export const ok = <const T, const E>(value: T): Result<T, E> =>
  new Result({ kind: 'ok', value });

export const err = <const T, const E>(reason: E): Result<T, E> =>
  new Result({ kind: 'err', reason });

export const Do: ResultComputation<{}, unknown> = new ResultComputation(
  new Result({ kind: 'ok', value: {} })
);
