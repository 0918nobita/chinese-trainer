import { useState, type FC } from 'react';

import { PinyinTextField } from '../../ui-components';

export const MainPage: FC = () => {
    const [pinyin, setPinyin] = useState('de');

    return <PinyinTextField pinyin={pinyin} onChange={setPinyin} />;
};
