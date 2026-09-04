const assert = require("node:assert/strict");
const vm = require("node:vm");

const extractBlock = (source, marker) => {
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `Missing source marker: ${marker}`);
  const bodyStart = source.indexOf("{", start);
  assert.notEqual(bodyStart, -1, `Missing function body: ${marker}`);
  let depth = 0;
  for (let i = bodyStart; i < source.length; i++) {
    if (source[i] === "{") depth += 1;
    if (source[i] === "}") depth -= 1;
    if (depth === 0) return source.slice(start, i + 1);
  }
  throw new Error(`Unclosed function body: ${marker}`);
};

const loadNamedFunction = (source, name, context) => {
  const asyncMarker = `async function ${name}`;
  const marker = source.includes(asyncMarker)
    ? asyncMarker
    : `function ${name}`;
  const declaration = extractBlock(source, marker);
  vm.runInNewContext(`${declaration}; globalThis.__test = ${name};`, context);
  return context.__test;
};

const loadArrowFunction = (source, name, context) => {
  const start = source.indexOf(`const ${name} =`);
  assert.notEqual(start, -1, `Missing function declaration: ${name}`);
  for (
    let end = source.indexOf(";", start);
    end !== -1;
    end = source.indexOf(";", end + 1)
  ) {
    let script;
    try {
      script = new vm.Script(source.slice(start, end + 1));
    } catch (error) {
      if (error instanceof SyntaxError) continue;
      throw error;
    }
    script.runInNewContext(context);
    return vm.runInNewContext(name, context);
  }
  throw new Error(`Unclosed function declaration: ${name}`);
};

module.exports = { loadArrowFunction, loadNamedFunction };
