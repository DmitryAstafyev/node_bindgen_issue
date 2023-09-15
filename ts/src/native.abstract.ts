import { getNativeModule } from "./native";

export abstract class Native {
  public abstract init(): Promise<void>;

  public abstract sleep(delay: number): Promise<void>;
}

export class Jobs {
  protected readonly native: Native;

  constructor() {
    this.native = new (getNativeModule().Jobs)() as Native;
    console.log(`TS: Rust Jobs native session is created`);
  }

  public async init(): Promise<Jobs> {
    return new Promise((resolve, reject) => {
      this.native
        .init()
        .then(() => {
          console.log(`TS: Rust Jobs native session is inited`);
          resolve(this);
        })
        .catch((err: Error) => {
          console.error(
            `TS: Fail to init Jobs session: ${
              err instanceof Error ? err.message : err
            }`
          );
          reject(err);
        });
    });
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
