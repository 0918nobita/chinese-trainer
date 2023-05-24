class Option<T> {
  constructor(private inner: T | null) {}

  static none = new Option<unknown>(null);

  bind<U>(f: (inner: T) => Option<U>): Option<U> {
    return this.inner === null ? (Option.none as Option<U>) : f(this.inner);
  }

  eq(other: Option<T>): boolean {
    return this.inner === other.inner;
  }

  map<U>(mapping: (inner: T) => U): Option<U> {
    return this.inner === null
      ? (Option.none as Option<U>)
      : new Option(mapping(this.inner));
  }

  match<U>(ifSome: (inner: T) => U, ifNone: () => U): U {
    return this.inner === null ? ifNone() : ifSome(this.inner);
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

export type { Option };

export const none = <A>(): Option<A> => Option.none as Option<A>;

export const some = <A>(inner: A): Option<A> => new Option(inner);

export const fromNullable = <A>(
  nullableValue: A | null | undefined
): Option<A> =>
  nullableValue === null || nullableValue === undefined
    ? (Option.none as Option<A>)
    : some(nullableValue);

export const Do = (): OptionComputation<{}> =>
  new OptionComputation(new Option({}));
