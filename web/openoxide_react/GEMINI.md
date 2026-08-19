# Project Rules & Coding Standards

This file contains rules and guidelines that the coding assistant must follow when developing this project.

## 1. Component Reuse (Shadcn/UI)

- **Always check for prebuilt shadcn components** before writing custom UI components from scratch.
- If the required shadcn component is already installed in `src/components/ui/`, import and use it.
- If the component is not yet installed:
  1. Install it using the command:
     ```bash
     bunx --bun shadcn add <component-name>
     ```
  2. Use it in the code.
- Import shadcn components using the configured alias: `#/components/ui/<component-name>`.

## 2. Codebase Directory Structure & Modularity

- **No monolithic files**: Never bundle API logic, schemas, forms, and UI components in a single, massive file.
- Follow the layered-based codebase structure under `src/`:
  - **API Calls**: Under `src/api/` (e.g., `src/api/auth.ts`)
  - **Custom Hooks**: Under `src/hooks/` (e.g., `src/hooks/use-signin.ts`)
  - **Validation Schemas**: Under `src/schema/` (e.g., `src/schema/signin.schema.ts`)
  - **Forms**: Under `src/components/forms/` (e.g., `src/components/forms/signin.form.tsx`)
  - **Layouts**: Under `src/components/layouts/` (e.g., `src/components/layouts/sidebar/`)
  - **Shared / Reusable UI Widgets**: Under `src/components/ui/` (for shadcn UI components) and `src/components/shared/` (for other shared UI widgets).

## 3. API & Code Generation (OpenAPI-TypeScript)

- **Auto-generated Types**: Always use the generated types from `src/types/api.d.ts` (`#/types/api.d.ts`). Do not manually define interfaces or types for API responses or requests if they are already present in the OpenAPI schema.
- **Type-Safe API Calls**: Prefer using the type-safe `$api` client (from `#/api/query` using `openapi-react-query`) or `client` (from `#/api/client` using `openapi-fetch`) for calling API endpoints. They automatically enforce path, parameters, request body, and response types from the schema.
- **Regenerating Schema**: If the API endpoints are updated on the backend, run `bun run gen:api` to update the generated types file.

## 4. File and Component Size Limits (Max 200 Lines)

- **Strict 200-Line Limit**: If any component, hook, or file exceeds 200 lines, it **must** be refactored and split into smaller sub-components, custom hooks, or helper modules.
- **Recursive Splitting**: Even if a file has already been split, if any of the sub-components or files grow beyond 200 lines, they must be split further.
- **Keep Code Clean**: Never allow massive, monolithic files with mixed state, query logic, and complex rendering.
