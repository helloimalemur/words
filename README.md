This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

## Download and install (requires Rust and pnpm)
```shell
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/helloimalemur/words/master/install.sh | sh
```

## Development server:
```bash
export WEBKIT_DISABLE_COMPOSITING_MODE=1; pnpm tauri dev;
```

```shell
export NO_STRIP=true; cargo tauri build --verbose;
#cargo tauri build --verbose --target x86_64-apple-darwin
```

## troubleshooting
```shell
## blank/empty window on nvidia based systems
export WEBKIT_DISABLE_COMPOSITING_MODE=1;
## zombie frontend process
for PORT in {3000..3002}; do kill -9 $(netstat -tulpn | grep $PORT | tr -s ' ' | cut -d '/' -f1 | cut -d ' ' -f7); done;
```
