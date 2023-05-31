import * as grpc from "@grpc/grpc-js";
import { createSchema, createYoga } from "graphql-yoga";
import { readFile } from "node:fs/promises";
import { createServer } from "node:http";
import path from "node:path";

import type { Resolvers } from "../graphql/generated/bff.d.ts";
import { WordServiceClient } from "./generated/word_grpc_pb";
import { WordRequest, WordResponse } from "./generated/word_pb";

const grpcClient = new WordServiceClient(
  "localhost:50051",
  grpc.credentials.createInsecure()
);

function getWordById(id: string): Promise<WordResponse> {
  const request = new WordRequest();
  request.setId(id);
  return new Promise((resolve, reject) => {
    grpcClient.getWord(request, (err, res) => {
      if (err) {
        reject(err);
      } else {
        resolve(res);
      }
    });
  });
}

async function main() {
  const typeDefs = await readFile(
    path.resolve(__dirname, "../graphql/schema.graphql"),
    "utf-8"
  );

  const resolvers: Resolvers = {
    Query: {
      Word: async (_parent, { id: rawId }, _context, _info) => {
        const id = String(rawId);
        const word = await getWordById(id);
        return {
          id,
          simplified_chinese_characters: word.getSimplfiedChineseCharacters(),
          pinyin: word.getPinyin(),
          meaning: word.getMeaning(),
        };
      },
    },
  };

  const yoga = createYoga({
    schema: createSchema({
      typeDefs,
      resolvers,
    }),
  });

  const server = createServer(yoga);

  server.listen(8000, () => {
    console.info(
      `Server listening on http://localhost:8000${yoga.graphqlEndpoint}`
    );
  });
}

void main();
