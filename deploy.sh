# login to npm
echo "//registry.npmjs.org/:_authToken=$NPM_TOKEN" > ~/.npmrc

# run build script
./build.sh

# publish
wasm-pack publish