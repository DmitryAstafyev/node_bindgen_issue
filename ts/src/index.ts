import { Jobs } from "./native.abstract";

const jobs = new Jobs();

async function run() {
  await jobs.init();
  console.log(`TS: Session is inited. Calling sleep`);
  await jobs.sleep(4000);
  console.log(`TS: Sleep is done. Session has been destroyed`);
}

run().catch((err: Error) => {
  console.error(err.message);
});
