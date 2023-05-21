export class Option<T> {
  private constructor(private inner: T | null) {}

  public eq(other: Option<T>): boolean {
    return this.inner === other.inner;
  }

  public map<U>(mapping: (inner: T) => U): Option<U> {
    if (this.inner === null) return this as unknown as Option<U>;
    return new Option<U>(mapping(this.inner));
  }

  static some<A>(value: A): Option<A> {
    return new Option(value);
  }

  static none<A>(): Option<A> {
    return new Option<A>(null);
  }

  static fromNullable<A>(nullableValue: A | undefined | null): Option<A> {
    if (nullableValue === undefined || nullableValue === null)
      return Option.none();
    return Option.some(nullableValue);
  }
}
