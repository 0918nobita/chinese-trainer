import { describe, expect, it, vi } from 'vitest';

import * as O from './option';
import * as R from './result';

describe('eq', () => {
  it('when both are None', () => {
    const a = O.none();
    const b = O.none();
    expect(a.eq(b)).toBe(true);
  });

  it('when lhs is None and rhs is Some(v)', () => {
    const a = O.none<number>();
    const b = O.some(1);
    expect(a.eq(b)).toBe(false);
  });

  it('when lhs is Some(v) and rhs is None', () => {
    const a = O.some(1);
    const b = O.none<1>();
    expect(a.eq(b)).toBe(false);
  });

  it('when inner values are different', () => {
    const a = O.some<number>(1);
    const b = O.some<number>(2);
    expect(a.eq(b)).toBe(false);
  });

  it('when inner values are the same', () => {
    const a = O.some(1);
    const b = O.some(1);
    expect(a.eq(b)).toBe(true);
  });
});

describe('map', () => {
  it('map None', () => {
    const fn = vi.fn<[number], number>((x) => x + 1);
    const opt = O.none<number>().map(fn);

    expect(opt.eq(O.none())).toBe(true);
    expect(fn).not.toBeCalled();
  });

  it('map Some(v)', () => {
    const opt = O.some(24).map((x) => x / 3);
    expect(opt.eq(O.some(8))).toBe(true);
  });
});

describe('match', () => {
  it('None', () => {
    const ifSome = vi.fn<[number], number>((_x) => 1);
    const ifNone = vi.fn<[], number>(() => 2);
    const result = O.none<number>().match(ifSome, ifNone);

    expect(result).toBe(2);
    expect(ifSome).not.toBeCalled();
    expect(ifNone).toBeCalledTimes(1);
  });

  it('Some(v)', () => {
    const ifSome = vi.fn<[number], number>((_x) => 1);
    const ifNone = vi.fn<[], number>(() => 2);
    const result = O.some(42).match(ifSome, ifNone);

    expect(result).toBe(1);
    expect(ifSome).toBeCalledTimes(1);
    expect(ifNone).not.toBeCalled();
  });
});

describe('okOr', () => {
  it('None', () => {
    const result = O.none().okOr('err');
    expect(result.eq(R.err('err'))).toBe(true);
  });

  it('Some(v)', () => {
    const result = O.some(12).okOr('err');
    expect(result.eq(R.ok(12))).toBe(true);
  });
});

describe('unwrapOr', () => {
  it('None', () => {
    const n = O.none<number>().unwrapOr(12);
    expect(n).toBe(12);
  });

  it('Some(v)', () => {
    const n = O.some<number>(7).unwrapOr(12);
    expect(n).toBe(7);
  });
});

describe('unwrapOrElse', () => {
  it('None', () => {
    const fn = vi.fn<[], number>(() => 12);
    const n = O.none<number>().unwrapOrElse(fn);

    expect(fn).toBeCalledTimes(1);
    expect(n).toBe(12);
  });

  it('Some(v)', () => {
    const fn = vi.fn<[], number>(() => 12);
    const n = O.some<number>(7).unwrapOrElse(fn);

    expect(fn).not.toBeCalled();
    expect(n).toBe(7);
  });
});

describe('fromNullable', () => {
  it('from null', () => {
    const opt = O.fromNullable(null);
    expect(opt.eq(O.none())).toBe(true);
  });

  it('from undefined', () => {
    const opt = O.fromNullable(undefined);
    expect(opt.eq(O.none())).toBe(true);
  });

  it('from a value which is neither null nor undefined', () => {
    const opt = O.fromNullable(24);
    expect(opt.eq(O.some(24))).toBe(true);
  });
});

describe('do', () => {
  it('bind/return', () => {
    const opt = O.Do.bind('a', () => O.some(1))
      .bind('b', ({ a }) => O.some(a + 2))
      .return(({ a, b }) => a + b);

    expect(opt.eq(O.some(4))).toBe(true);
  });

  it('when None is returned during execution of method chain', () => {
    const fn = vi.fn<[{ a: number }], number>(({ a }) => a + 7);

    const opt = O.Do.bind('a', () => O.none<number>()).return(fn);

    expect(fn).not.toBeCalled();
    expect(opt.eq(O.none())).toBe(true);
  });

  it('returnFrom', () => {
    const opt = O.Do.returnFrom(() => O.some(24));

    expect(opt.eq(O.some(24))).toBe(true);
  });
});
