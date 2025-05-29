# Example Rust Embed SvelteKit

A demonstration project to show how to serve an SPA from the backend.

![](demo.gif)

Technical stack:

- Moon: task runner and monorepo management
- Rust + Axum: backend
- SvelteKit + DaisyUI + TailwindCSS: frontend

## Development

- Make sure that you have related tools available:

  - `moon`
  - `cargo` and `rustc`
  - `pnpm`

- Install dependencies:

```shell
moon run backend:install
moon run frontend:install
```

- Start the servers:
  - Backend:

  ```shell
  moon run backend:serve
  ```

  - Frontend building:

  ```shell
  moon run frontend:serve
  ```

## Production Build

```shell
moon run backend:build
```

Test starting binary:

```shell
./packages/backend/target/release/backend
# Listening on http://127.0.0.1:3000
```
