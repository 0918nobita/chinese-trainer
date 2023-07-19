declare const tag: unique symbol;

export type EmptyObject = { [tag]?: never };
