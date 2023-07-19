import { type ChangeEvent, type FC, useCallback, useState } from 'react';

import { ToneSelector } from '..';

import { label, pinyinInput } from './style.css';

type Props = Readonly<{
    pinyin: string;
    onChange: (value: string) => void;
}>;

export const PinyinTextField: FC<Props> = ({ pinyin, onChange }) => {
    const handleInputChange = (e: ChangeEvent<HTMLInputElement>) => {
        onChange(e.target.value);
    };

    const [visible, setVisibility] = useState(false);

    const handleInputFocus = useCallback(() => {
        setVisibility(true);
    }, []);

    const onClose = useCallback(() => {
        setVisibility(false);
    }, []);

    const onSelect = useCallback((tone: 0 | 1 | 2 | 3 | 4) => {
        console.log(tone);
    }, []);

    return (
        <>
            <label className={label}>
                Pinyin:&nbsp;
                <input
                    type="text"
                    className={pinyinInput}
                    value={pinyin}
                    onChange={handleInputChange}
                    onFocus={handleInputFocus}
                />
            </label>
            {visible && <ToneSelector onClose={onClose} onSelect={onSelect} />}
        </>
    );
};
