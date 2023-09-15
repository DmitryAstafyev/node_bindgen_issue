import { getNativeModule } from "./native";

export abstract class Native {
  public abstract init(): Promise<void>;

  public abstract destroy(): Promise<void>;

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

  public async destroy(): Promise<void> {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        console.error(`TS: Timeout error. Session wasn't closed in 5 sec.`);
        reject(new Error(`Timeout error. Session wasn't closed in 5 sec.`));
      }, 5000);
      this.native
        .destroy()
        .then(() => {
          console.log(`TS: Session has been destroyed`);
          resolve();
        })
        .catch((err: Error) => {
          console.error(
            `TS: Fail to close session due error: ${
              err instanceof Error ? err.message : err
            }`
          );
          reject(err);
        })
        .finally(() => {
          clearTimeout(timeout);
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
