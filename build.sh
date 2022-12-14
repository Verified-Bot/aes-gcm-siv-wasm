# build package
wasm-pack build

# remove last line from package.json fine
LC=`wc -l < ./pkg/package.json`
LC=$(($LC - 2 ))
head -n $LC ./pkg/package.json > ./pkg/package_n.json
rm ./pkg/package.json
mv ./pkg/package_n.json ./pkg/package.json

# add module type to pakcage.json
echo "  \"type\": \"module\"," >> pkg/package.json
echo "}" >> pkg/package.json