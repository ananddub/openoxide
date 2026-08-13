#!/usr/bin/env node
import {generateLiveDeclarations} from './index.js';

const [root, manifestPath, declarations, manifestBin] = process.argv.slice(2);
const result = generateLiveDeclarations({
  root: root ?? process.cwd(),
  manifestPath,
  declarations,
  manifestBin,
});

console.log(`[openoxide] generated ${result.endpoints} live hooks`);
console.log(`[openoxide] declarations: ${result.declarations}`);
