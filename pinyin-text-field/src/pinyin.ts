import * as O from './option';

export type Consonant = 'b' | 'p' | 'f' | 'm' | 'd' | 't' | 'n' | 'l' | 'g' | 'k' | 'h' | 'j' | 'q' | 'x' | 'zh' | 'ch' | 'sh' | 'r' | 'z' | 'c' | 's';

export type Medial = 'i' | 'u' | 'ü';

export type Vowel = 'a' | 'e' | 'i' | 'u' | 'ü';

export type Coda = 'i' | 'o' | 'u' | 'n' | 'ng';

export type Tone = 1 | 2 | 3 | 4 | 5;

export type Syllable = {
    consonant: O.Option<Consonant>;
    medial: O.Option<Medial>;
    vowel: Vowel;
    coda: O.Option<Coda>;
    tone: Tone;
};

export type Pinyin = readonly Syllable[];
