nj-cli build --release
../ts/node_modules/.bin/tsc -p ../ts/tsconfig.json
cp ./dist/index.node ../ts/dist/index.node