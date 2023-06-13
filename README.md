# Makepad TODO list application

This is an example application of simple Todo list application built with [Makepad](https://github.com/makepad/makepad)

## Testing it in Android

Install cargo-makepad, in the root of the Makepad repository run

```
cargo install --path ./tools/cargo_makepad
```

Note: Make sure you have checked in the latest from Makepad's main branch.

Run the application for Android 
```
cargo-makepad android run -p todo_makepad
```

## Testing it in other platforms

Run the application for Web (Wasm)

```
../makepad/tools/wasm/build_wasm_thread.sh todo_makepad; makepad-web-server
```

Note: The `build_wasm_thread.sh` is actually located where you have checked in Makepad, so please adjust the relative path as it fits in your situation.

Run the application for Desktop (MacOS) 

```
cargo run
```

### Notes

* There are two version of the application, one for mobile (`AppMobile`) and other for desktop (`AppDesktop`). You need to choose manually in src/app.rs which want to render, since there is no an automatic mechanism for that at the time.

* This currently uses a JSON API deployed at https://cholee-todo-app.fly.dev/api/todos/. Random test data will appear since this resource is shared.
