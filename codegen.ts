import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "graphql/schema.graphql",
  generates: {
    "./resolvers-types.d.ts": {
      config: {
        enumsAsTypes: true,
        useTypeImports: true,
      },
      plugins: ["typescript", "typescript-resolvers"],
    },
  },
};

export default config;
