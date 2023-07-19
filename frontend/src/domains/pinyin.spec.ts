import { assert, describe, expect, it } from 'vitest';
import { parsers } from './pinyin';

describe('parser', () => {
    it('consonant', () => {
        const consonant = parsers.consonant;

        const parseResult = consonant.parse('d');
        assert(parseResult.success);
        expect(parseResult.value).toBe('d');

        const parseResult2 = consonant.parse('u');
        assert(parseResult2.success === false);
    });

    it('tone', () => {
        const tone = parsers.tone;

        for (let toneNum = 0; toneNum < 5; toneNum++) {
            const parseResult = tone.parse(String(toneNum));
            assert(parseResult.success);
            expect(parseResult.value).toBe(toneNum);
        }

        const parseResult = tone.parse('5');
        assert(parseResult.success === false);
    });
});
