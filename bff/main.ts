import { createSchema, createYoga } from "npm:graphql-yoga@3.9.1";
import { serve } from "https://deno.land/std@0.190.0/http/server.ts";

const yoga = createYoga({
  schema: createSchema({
    typeDefs: `
      type Query {
        hello: String!
      }`,
    resolvers: {
      Query: {
        hello: () => "Hello, world!",
      },
    },
  }),
});

serve(yoga, {
  onListen({ hostname, port }) {
    console.log(
      `Listening on http://${hostname}:${port}/${yoga.graphqlEndpoint}`,
    );
  },
});
