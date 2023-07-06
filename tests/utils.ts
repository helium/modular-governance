import { execSync } from "child_process";

export async function ensureIdls() {
  let programs = [
    {
      name: "proposal",
      pid: "propXxHSbYTCSwqJA2Vv3Sw27LTJbhBQLSGmmUVZghq",
    },
    {
      name: "state_controller",
      pid: "resL1j3p3QXAD2oQWW14Uv18iJrfoAwrCd3qTd2QDyj",
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
