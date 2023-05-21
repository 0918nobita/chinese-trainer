import { describe, expect, it } from 'vitest';

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
