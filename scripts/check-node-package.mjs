// Install the actual packed artifact into a clean consumer, with no Rust build step.
import {execFileSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {resolve,join} from "node:path";
import {fileURLToPath} from "node:url";
const root=resolve(fileURLToPath(new URL("..",import.meta.url)));
const packageDir=join(root,"crates/node");
const temp=mkdtempSync(join(tmpdir(),"aux-ts-package-"));
try {
 const packed=JSON.parse(execFileSync("npm",["pack","--json","--pack-destination",temp],{cwd:packageDir,encoding:"utf8"}))[0];
 writeFileSync(join(temp,"package.json"),'{"private":true,"type":"module"}');
 execFileSync("npm",["install","--ignore-scripts","--no-audit","--no-fund",join(temp,packed.filename)],{cwd:temp,stdio:"inherit"});
 execFileSync(process.execPath,["--input-type=module","-e",`import {createRequire} from 'node:module'; import {HyperliquidClient,scaleLighterDecimal} from 'aux-ts'; const cjs=createRequire(import.meta.url)('aux-ts');if(cjs.HyperliquidClient!==HyperliquidClient)throw Error('exports differ'); const client=new HyperliquidClient();await client.close();if(scaleLighterDecimal({value:'1.23',decimals:2})!==123n)throw Error('incorrect native result');`],{cwd:temp,stdio:"inherit"});
 console.log("Installed package: ESM and CommonJS passed");
} finally {rmSync(temp,{recursive:true,force:true});}
