export class Option<T> {
  private constructor(private inner: T | null) {}

  static #none = new Option<unknown>(null);

  bind<U>(f: (inner: T) => Option<U>): Option<U> {
    return this.inner === null ? (Option.#none as Option<U>) : f(this.inner);
  }

  eq(other: Option<T>): boolean {
    return this.inner === other.inner;
  }

  map<U>(mapping: (inner: T) => U): Option<U> {
    return this.inner === null
      ? (Option.#none as Option<U>)
      : Option.some(mapping(this.inner));
  }

  match<U>(ifSome: (inner: T) => U, ifNone: () => U): U {
    return this.inner === null ? ifNone() : ifSome(this.inner);
  }

  static some<A>(value: A): Option<A> {
    return new Option(value);
  }

  static none<A>(): Option<A> {
    return Option.#none as Option<A>;
  }

  static fromNullable<A>(nullableValue: A | undefined | null): Option<A> {
    return nullableValue === undefined || nullableValue === null
      ? (Option.#none as Option<A>)
      : Option.some(nullableValue);
  }

  static do(): OptionComputation<{}> {
    return new OptionComputation(Option.some({}));
  }
}

class OptionComputation<T> {
  constructor(private currentOpt: Option<T>) {}

  bind<const Bound extends string, A>(
    bound: Bound,
    f: (inner: T) => Option<A>
  ): OptionComputation<{
    [K in Bound | keyof T]: K extends Bound
      ? A
      : K extends keyof T
      ? T[K]
      : never;
  }> {
    return new OptionComputation(
      this.currentOpt.bind((bindings) =>
        f(bindings).map(
          (a) =>
            ({ ...bindings, [bound]: a } as {
              [K in Bound | keyof T]: K extends Bound
                ? A
                : K extends keyof T
                ? T[K]
                : never;
            })
        )
      )
    );
  }

  return<A>(f: (bindings: T) => A): Option<A> {
    return this.currentOpt.map(f);
  }

  returnFrom<A>(f: (bindings: T) => Option<A>): Option<A> {
    return this.currentOpt.bind(f);
  }
}
