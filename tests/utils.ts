import { execSync } from "child_process";

export const ANCHOR_PATH = process.env.ANCHOR_PATH || "anchor";

export async function ensureIdls() {
  let programs = [
    {
      name: "proposal",
      pid: "propFYxqmVcufMhk5esNMrexq2ogHbbC2kP9PU1qxKs",
    },
    {
      name: "state_controller",
      pid: "stcfiqW3fwD9QCd8Bqr1NBLrs7dftZHBQe7RiMMA4aM",
    },
    {
      name: "token_voter",
      pid: "tokvN2E37T6NgLi6uQ8uj32959TZPUf2Jo8dXjLKBjF",
    },
    {
      name: "nft_voter",
      pid: "nftvLQ5t6xe2nQF1NBmBBmn15ed59tU6vSCkwQNEqdc",
    },
    {
      name: "nft_proxy",
      pid: "nprx42sXf5rpVnwBWEdRg1d8tuCWsTuVLys1pRWwE6p",
    },
    {
      name: "organization",
      pid: "orgdXvHVLkWgBYerptASkAwkZAE563CJUu717dMNx5f",
    },
    {
      name: "organization_wallet",
      pid: "orgwPMqJs9xft8UefUdKfyBwg6GDnN6oLhpMaKa6nJg",
    },
  ];
  await Promise.all(
    programs.map(async (program) => {
      try {
        execSync(
          `${ANCHOR_PATH} idl init --filepath ${__dirname}/../target/idl/${program.name}.json ${program.pid}`,
          { stdio: "inherit", shell: "/bin/bash" }
        );
      } catch {
        execSync(
          `${ANCHOR_PATH} idl upgrade --filepath ${__dirname}/../target/idl/${program.name}.json ${program.pid}`,
          { stdio: "inherit", shell: "/bin/bash" }
        );
      }
    })
  );
}

export function makeid(length: number) {
  let result = "";
  const characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  const charactersLength = characters.length;
  let counter = 0;
  while (counter < length) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
    counter += 1;
  }
  return result;
}