import { readdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import addFormats from "ajv-formats";

const here = path.dirname(fileURLToPath(import.meta.url));
const eventsDir = path.resolve(here, "../../../contracts/events");
const fixturesDir = path.join(eventsDir, "fixtures");
const schemaFiles = (await readdir(eventsDir)).filter((file) =>
  file.endsWith(".schema.json"),
);
const fixtureFiles = (await readdir(fixturesDir)).filter((file) =>
  file.endsWith(".json"),
);
const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);

for (const file of schemaFiles) {
  ajv.addSchema(await Bun.file(path.join(eventsDir, file)).json());
}

let failures = 0;
for (const file of fixtureFiles) {
  const matched = /^(.*)\.(valid|invalid)\.json$/.exec(file);
  if (!matched) {
    throw new Error(`fixture 名称不符合约定：${file}`);
  }
  const [, stem, expected] = matched;
  const schemaId = `https://ecommerce-core-reference.local/events/${stem}.schema.json`;
  const validate = ajv.getSchema(schemaId);
  if (!validate) {
    throw new Error(`fixture 缺少声明 schema：${file}`);
  }
  const actual = validate(await Bun.file(path.join(fixturesDir, file)).json());
  if (actual !== (expected === "valid")) {
    failures += 1;
    console.error(
      `${file}: 期望 ${expected}，实际 ${actual ? "valid" : "invalid"}`,
    );
  }
}

if (failures > 0) {
  process.exitCode = 1;
}
