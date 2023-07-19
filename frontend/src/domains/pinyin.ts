import * as Te from 'terrario';

import { mapTuple } from '../utilities/mapTuple';
import * as O from '../utilities/option';

export const consonant = [
    'b',
    'p',
    'f',
    'm',
    'd',
    't',
    'n',
    'l',
    'g',
    'k',
    'h',
    'j',
    'q',
    'x',
    'zh',
    'ch',
    'sh',
    'r',
    'z',
    'c',
    's',
] as const;

export type Consonant = (typeof consonant)[number];

export const medial = ['i', 'u', 'ü'] as const;

export type Medial = (typeof medial)[number];

export const vowel = ['a', 'e', 'i', 'u', 'ü'] as const;

export type Vowel = (typeof vowel)[number];

export const coda = ['i', 'o', 'u', 'n', 'ng'] as const;

export type Coda = (typeof coda)[number];

export type Tone = 0 | 1 | 2 | 3 | 4;

export type Syllable = {
    consonant: O.Option<Consonant>;
    medial: O.Option<Medial>;
    vowel: Vowel;
    coda: O.Option<Coda>;
    tone: Tone;
};

export type Pinyin = readonly Syllable[];

const strTupleToAltParser = <const Tuple extends readonly string[]>(
    tuple: Tuple
): Te.Parser<Tuple[number]> =>
    Te.alt(
        mapTuple(tuple, (str) => Te.str(str)) as unknown as Te.Parser<Tuple>[]
    );

const consonantParser = strTupleToAltParser(consonant);

const medialParser = strTupleToAltParser(medial);

const vowelParser = strTupleToAltParser(vowel);

const codaParser = strTupleToAltParser(coda);

const toneParser = Te.alt([
    Te.str('0'),
    Te.str('1'),
    Te.str('2'),
    Te.str('3'),
    Te.str('4'),
]).map((str) => parseInt(str));

// TODO: 声調記号もパースできるようにする
const syllableParser = Te.seq([
    consonantParser.option(),
    medialParser.option(),
    vowelParser,
    codaParser.option(),
]);

export const parsers = {
    consonant: consonantParser,
    medial: medialParser,
    vowel: vowelParser,
    coda: codaParser,
    tone: toneParser,
    syllable: syllableParser,
};
