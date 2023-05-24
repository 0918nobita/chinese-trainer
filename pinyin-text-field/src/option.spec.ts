import { describe, expect, it, vi } from 'vitest';

import { Option } from './option';

describe('eq', () => {
  it('when both are None', () => {
    const a = Option.none();
    const b = Option.none();
    expect(a.eq(b)).toBe(true);
  });

  it('when lhs is None and rhs is Some(v)', () => {
    const a = Option.none<number>();
    const b = Option.some(1);
    expect(a.eq(b)).toBe(false);
  });

  it('when lhs is Some(v) and rhs is None', () => {
    const a = Option.some(1);
    const b = Option.none<number>();
    expect(a.eq(b)).toBe(false);
  });

  it('when inner values are different', () => {
    const a = Option.some(1);
    const b = Option.some(2);
    expect(a.eq(b)).toBe(false);
  });

  it('when inner values are the same', () => {
    const a = Option.some(1);
    const b = Option.some(1);
    expect(a.eq(b)).toBe(true);
  });
});

it('map', () => {
  expect(
    Option.none<number>()
      .map((x) => x + 1)
      .eq(Option.none())
  ).toBe(true);
});

describe('do', () => {
  it('bind/return', () => {
    const opt = Option.do()
      .bind('a', () => Option.some(1))
      .bind('b', ({ a }) => Option.some(a + 2))
      .return(({ a, b }) => a + b);

    expect(opt.eq(Option.some(4))).toBe(true);
  });

  it('when None is returned during execution of method chain', () => {
    const fn = vi
      .fn<[{ a: number }], number>()
      .mockImplementation(({ a }) => a + 7);

    const opt = Option.do()
      .bind('a', () => Option.none<number>())
      .return(fn);

    expect(fn).toBeCalledTimes(0);

    expect(opt.eq(Option.none())).toBe(true);
  });

  it('returnFrom', () => {
    const opt = Option.do().returnFrom(() => Option.some(24));

    expect(opt.eq(Option.some(24))).toBe(true);
  });
});
