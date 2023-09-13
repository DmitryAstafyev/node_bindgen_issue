import * as path from "path";
import * as fs from "fs";

export interface IRustModuleExports {
  Jobs: any;
}

export function getNativeModule(): IRustModuleExports {
  const modulePath = (() => {
    const paths = [
      path.resolve(module.path, "./index.node"),
      // This path is actual for Jasmine tests use-cases
      path.resolve(module.path, "../../../../src/native/index.node"),
    ];
    for (const target of paths) {
      if (fs.existsSync(target)) {
        return target;
      }
    }
    throw new Error(`Fail to find modules in:\n${paths.join("\n")}`);
  })();
  return require(modulePath);
}

const { Jobs: RustSessionNoType } = getNativeModule();

export { RustSessionNoType };
