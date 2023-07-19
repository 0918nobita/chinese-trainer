export const mapTuple = <const Tuple extends readonly unknown[], const Mapped>(
    tuple: Tuple,
    mapping: (elem: Tuple[number]) => Mapped
) => tuple.map(mapping) as { [K in keyof Tuple]: Mapped };
