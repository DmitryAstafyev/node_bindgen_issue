import { Jobs } from "./native.abstract";

const jobs = new Jobs();

async function run() {
  console.log(`TS: Calling sleep`);
  await jobs.sleep(4000);
  console.log(`TS: Sleep is done.`);
}

run().catch((err: Error) => {
  console.error(err.message);
});
