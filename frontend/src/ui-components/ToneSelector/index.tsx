import { type FC } from 'react';

import { className } from './style.css';

type Tone = 0 | 1 | 2 | 3 | 4;

type Props = Readonly<{
    onClose: () => void;
    onSelect: (tone: Tone) => void;
}>;

export const ToneSelector: FC<Props> = ({ onClose, onSelect }) => {
    const onClickCloseButton = () => {
        onClose();
    };

    const onSelectTone0 = () => {
        onSelect(0);
    };

    const onSelectTone1 = () => {
        onSelect(1);
    };

    const onSelectTone2 = () => {
        onSelect(2);
    };

    const onSelectTone3 = () => {
        onSelect(3);
    };

    const onSelectTone4 = () => {
        onSelect(4);
    };

    return (
        <div aria-hidden className={className}>
            <button type="button" onClick={onClickCloseButton}>
                閉じる
            </button>

            <button type="button" onClick={onSelectTone0}>
                軽声
            </button>

            <button type="button" onClick={onSelectTone1}>
                1声
            </button>

            <button type="button" onClick={onSelectTone2}>
                2声
            </button>

            <button type="button" onClick={onSelectTone3}>
                3声
            </button>

            <button type="button" onClick={onSelectTone4}>
                4声
            </button>
        </div>
    );
};
