### Build
```
wasm-pack build
```

### Put it into a website

This creates a `www` directory

```
npm init wasm-app www
cd www
```


### Modify the `package.json` file to use the local wasm file

```
{
  // ...
  "dependencies": {                     // Add this three lines block!
    "wasm-game-of-life": "file:../pkg"
  },
  "devDependencies": {
    //...
  }
}

```

### Install NPM dependencies
```
# From within the www directory
npm install
```


### 🛠️ Build with `wasm-pack build`

```
# From within the www directory
wasm-pack build
```

### Serve using Webpack
```
# From within the www directory
npm start
# Go to [http://localhost:8080](http://localhost:8080)
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```
