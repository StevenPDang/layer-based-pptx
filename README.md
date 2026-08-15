# layer-based-pptx
Powerpoint editor with editing capabilites similar to figma and photoshop

## Desktop development

Install dependencies and start the Electron application:

```bash
pnpm --dir apps/desktop install
pnpm dev
```

The React renderer remains available in a browser with `pnpm dev:web`. Build
the production renderer with `pnpm build`, then launch it in Electron with
`pnpm start`.

## requirements

- node.js = `latest`
- pnpm
- docker >= `3`

## Project Setup

1. Clone repo locally
2. Install dependencies
```
pnpm install
```
3. Intialize docker container
```
pnpm docker
```
