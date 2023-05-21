export class Option<T> {
  private constructor(private inner: T | null) {}

  static #none = new Option<unknown>(null);

  public eq(other: Option<T>): boolean {
    return this.inner === other.inner;
  }

  public map<U>(mapping: (inner: T) => U): Option<U> {
    if (this.inner === null) return Option.#none as Option<U>;
    return Option.some(mapping(this.inner));
  }

  static some<A>(value: A): Option<A> {
    return new Option(value);
  }

  static none<A>(): Option<A> {
    return Option.#none as Option<A>;
  }

  static fromNullable<A>(nullableValue: A | undefined | null): Option<A> {
    if (nullableValue === undefined || nullableValue === null)
      return Option.#none as Option<A>;
    return Option.some(nullableValue);
  }
}
