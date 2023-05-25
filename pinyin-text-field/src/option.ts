class Option<const T> {
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

  unwrapOr(defaultValue: T): T {
    return this.inner === null ? defaultValue : this.inner;
  }

  unwrapOrElse(f: () => T): T {
    return this.inner === null ? f() : this.inner;
  }
}

class OptionComputation<const T> {
  constructor(private currentOpt: Option<T>) {}

  bind<const Bound extends string, A>(
    bound: Bound,
    f: (inner: T) => Option<A>
  ): OptionComputation<T & { [_ in Bound]: A }> {
    return new OptionComputation(
      this.currentOpt.bind((bindings) =>
        f(bindings).map((a) => {
          const newEntry = { [bound]: a } as { [_ in Bound]: A };
          return { ...bindings, ...newEntry };
        })
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

export const none = <const A>(): Option<A> => Option.none as Option<A>;

export const some = <const A>(inner: A): Option<A> => new Option(inner);

export const fromNullable = <const A>(
  nullableValue: A | null | undefined
): Option<A> =>
  nullableValue === null || nullableValue === undefined
    ? (Option.none as Option<A>)
    : new Option(nullableValue);

export const Do: OptionComputation<{}> = new OptionComputation(new Option({}));
