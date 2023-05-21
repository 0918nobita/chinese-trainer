import { Option } from './option';

describe('Option', () => {
  describe('eq', () => {
    test('when both are None', () => {
      const a = Option.none();
      const b = Option.none();
      expect(a.eq(b)).toBe(true);
    });

    test('when lhs is None and rhs is Some(v)', () => {
      const a = Option.none<number>();
      const b = Option.some(1);
      expect(a.eq(b)).toBe(false);
    });

    test('when lhs is Some(v) and rhs is None', () => {
      const a = Option.some(1);
      const b = Option.none<number>();
      expect(a.eq(b)).toBe(false);
    });

    test('when inner values are different', () => {
      const a = Option.some(1);
      const b = Option.some(2);
      expect(a.eq(b)).toBe(false);
    });

    test('when inner values are the same', () => {
      const a = Option.some(1);
      const b = Option.some(1);
      expect(a.eq(b)).toBe(true);
    });
  });

  test('map', () => {
    expect(
      Option.none<number>()
        .map((x) => x + 1)
        .eq(Option.none())
    ).toBe(true);
  });
});
