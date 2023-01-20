![Header](https://h4hn.de/hot/github/typescript-webpack-template-banner.png)

## Installation
### Quick Setup:
Simply click on the use template button and initialize your git repository.
Clone your respository to your device and initialize it with:

``` 
yarn install
```

### Traditional way:
Alternatively you can clone it via git. After that, just install the dependencies via yarn.

```
# Clone the repository
$ git clone https://github.com/teck-digital/typescript-webpack-template

# Go into the repository
$ cd typescript-webpack-template

# Install dependencies
$ yarn install
```

## Usage

There are four ways you can compile and run your code:

### Development
```
yarn dev
```

This will start a watch run, which watches for any file changes and then
recompiles. **After every compilation, the index.js file will be executed.**


### Production Test
```
yarn prod
```

This will compile your code production ready and execute the index.js file.


### Bundling your application

```
yarn build
```

This will compile your code production ready.

### Starting the app

```
yarn start
```
This will, by default, start your application using the _dist/index.js_ entrypoint.

**Important:** You need to have your application built before this script is effective.
If you want to save some time use `yarn prod`!


## Using NPM

If you prefer using npm over yarn, you are free to do so. 
There are two modifications you will have to make in order for it to work:

### 1. Delete yarn.lock

First, you should delete the `yarn.lock` file in your root directory, as it may
confuse editors or your computer into using yarn.

### 2. Install via npm

Now, install the dependencies and execute scripts via npm instead of yarn:

```
# Install dependencies
$ npm install

# Start development run
$ npm run dev
```

## Automated script exececution

### Modifying executed file

By default, when webpack config contains only one output file, that file will be run. 
If more files are present, a file named server.js or index.js (whichever exists) will be run.
If you need to modify this, you can pass the following option to your webpack config:

```javascript
// config/webpack.config.dev.js

... 

plugins: [
  new RunNodeWebpackPlugin({scriptToRun: "yourScript.js"}),
  ...
],

...
```

For more options see https://www.npmjs.com/package/run-node-webpack-plugin

#### Updating package.json

You have now successfully modified the webpack config. For production builds, however,
webpack is out of the game and node manually executes the compiled javascript file. 
You will have to update the `prod` and `start` script in your package.json accordingly:

```json
// package.json

"scripts": {
  "start": "node dist/yourScript.js",
  "prod": "... && node dist/yourScript.js",
},
```

### Opting out
If you want webpack to stop automatically executing your script after every
development build, you can remove the `RunNodeWebpackPlugin` like so:


```javascript
// config/webpack.config.dev.js

-- const RunNodeWebpackPlugin = require("run-node-webpack-plugin");

...

-- plugins: [new RunNodeWebpackPlugin(), new ESLintPlugin({extensions: ['js', 'ts']})],
++ plugins: [new ESLintPlugin({extensions: ['js', 'ts']})],

...
```

You may also want to remove the package from your dependencies afterwards: 
``` 
yarn remove run-node-webpack-plugin
```

**Important:** You will most likely have to update your package.json scripts too,
if you want to opt out entirely. See [Updating package.json](#updating-packagejson).


## Contributing
If you'd like to make this project better, more general or add a feature, feel free to open up 
a pull request. I am open to any new idea!

## Issues
Feel free to open an issue for any problem you encounter. I will try to help you
as soon as possible.

## License

This project is [MIT licensed](LICENSE).
