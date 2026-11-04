const {test}=require("node:test");
const assert=require("node:assert/strict");
const {pathToFileURL}=require("node:url");
const path=require("node:path");
test("CommonJS and ESM expose the same concrete classes",async()=>{
 const cjs=require(process.env.AUX_NODE_PACKAGE || "../js/index.cjs");
 const esm=await import(pathToFileURL(path.resolve(process.env.AUX_NODE_PACKAGE?path.join(process.env.AUX_NODE_PACKAGE,"js/index.mjs"):path.join(__dirname,"../js/index.mjs"))));
 assert.deepEqual(Object.keys(esm).sort(),Object.keys(cjs).sort());
 assert.equal(esm.HyperliquidClient,cjs.HyperliquidClient);
});
