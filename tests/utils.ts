import { execSync } from "child_process";

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
      pid: "nftvJPn25R8AM52AeQM7TxkN7CpgWvYVVEh5qgHPaQx",
    },
    {
      name: "organization",
      pid: "orgdXvHVLkWgBYerptASkAwkZAE563CJUu717dMNx5f",
    },
  ];
  await Promise.all(
    programs.map(async (program) => {
      try {
        execSync(
          `anchor idl init --filepath ${__dirname}/../target/idl/${program.name}.json ${program.pid}`,
          { stdio: "inherit", shell: "/bin/bash" }
        );
      } catch {
        execSync(
          `anchor idl upgrade --filepath ${__dirname}/../target/idl/${program.name}.json ${program.pid}`,
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