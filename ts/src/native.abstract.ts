import { getNativeModule } from "./native";

export abstract class Native {
  public abstract sleep(delay: number): Promise<void>;
}

export class Jobs {
  protected readonly native: Native;

  constructor() {
    this.native = new (getNativeModule().Jobs)() as Native;
    console.log(`TS: Rust Jobs native session is created`);
  }

  public async sleep(delay: number): Promise<void> {
    return new Promise((resolve, reject) => {
      this.native
        .sleep(delay)
        .then(resolve)
        .catch((err: Error) => {
          console.error(
            `TS: Fail to call operation "sleep" due error: ${
              err instanceof Error ? err.message : err
            }`
          );
          reject(err);
        });
    });
  }
}
