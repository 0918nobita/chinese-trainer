import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "./graphql/schema.graphql",

  generates: {
    "./graphql/generated/bff.d.ts": {
      plugins: ["typescript", "typescript-operations", "typescript-resolvers"],
    },

    "./graphql/generated/frontend.ts": {
      documents: ["./graphql/queries/*.graphql"],
      plugins: [
        "typescript",
        "typescript-operations",
        "typescript-graphql-request",
      ],
    },
  },
};

export default config;
