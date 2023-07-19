import { globalStyle, style } from '@vanilla-extract/css';

export const className = style({});

globalStyle(`${className} button`, {
    backgroundColor: '#9ca3af',
    border: 'none',
});
