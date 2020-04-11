[![Netlify Status](https://api.netlify.com/api/v1/badges/44f26fcc-34d9-43e2-8e88-4580b5a659ec/deploy-status)](https://app.netlify.com/sites/yew-todomvc/deploys)

## About

This template shows how to create a web app using Yew and wasm-pack.

## 🚴 Usage

### 🛠️ Build with `yarn run build`

```
yarn run build
```

### 🔬 Serve locally with `yarn run start:dev`

```
yarn run start:dev
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
