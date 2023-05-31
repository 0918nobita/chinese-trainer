import { createSchema, createYoga } from "npm:graphql-yoga@3.9.1";
import { serve } from "https://deno.land/std@0.190.0/http/server.ts";

import type { Resolvers } from "../graphql/generated/bff.d.ts";

const typeDefs = Deno.readTextFileSync("../graphql/schema.graphql");

const resolvers: Resolvers = {
  Query: {
    Word: (_parent, { id }, _context, _info) => ({
      id: String(id),
      simplified_chinese_characters: "你好",
      pinyin: "nĭ hăo",
      meaning: "こんにちは",
    }),
  },
};

const yoga = createYoga({
  schema: createSchema({
    typeDefs,
    resolvers,
  }),
});

serve(yoga, {
  onListen({ hostname, port }) {
    console.log(
      `Listening on http://${hostname}:${port}/${yoga.graphqlEndpoint}`,
    );
  },
});
