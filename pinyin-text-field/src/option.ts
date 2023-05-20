const optionType = Symbol('Option');

export type Option<T> = {
    type: typeof optionType;
    variant: 'some';
    value: T;
} | {
    type: typeof optionType;
    variant: 'none';
};

export const some = <T>(value: T): Option<T> => ({ type: optionType, variant: 'some', value });

export const none = <T>(): Option<T> => ({ type: optionType, variant: 'none' });

export const fromNullable = <T>(nullableVal: T | undefined | null): Option<T> => nullableVal !== undefined && nullableVal !== null ? some(nullableVal) : none();
