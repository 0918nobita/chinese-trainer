<script lang="ts">
  import { GraphQLClient } from 'graphql-request';

  import PinyinTextField from './PinyinTextField.svelte';
  import { getSdk } from '../../graphql/generated/frontend';

  const client = new GraphQLClient('http://localhost:8000/graphql');
  const sdk = getSdk(client);
  const wordPromise = sdk.getWordById({ id: '10' }).then((res) => res.Word);
</script>

{#await wordPromise then word}
  <p>簡体字：{word.simplified_chinese_characters}</p>
  <p>ピンイン：{word.pinyin}</p>
  <p>意味：{word.meaning}</p>
{:catch error}
  <p class:error>{error.message}</p>
{/await}

<PinyinTextField />

<style>
  :root {
    --pinyin-text-field-slate-100: #f1f5f9;
    --pinyin-text-field-gray-400: #9ca3af;
    --pinyin-text-field-blue-300: #93c5fd;
  }

  p {
    color: var(--pinyin-text-field-slate-100);
  }

  .error {
    color: red;
  }
</style>
