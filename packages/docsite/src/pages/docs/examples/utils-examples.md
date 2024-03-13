## Dependencies

```typescript
import { execSync } from "child_process"

export async function ensureIdls() {
  let programs = [
    {
      name: "proposal",
      pid: "propXxHSbYTCSwqJA2Vv3Sw27LTJbhBQLSGmmUVZghq",
    },
    {
      name: "state_controller",
      pid: "stct65Ut9aiwQ5pQNSdD4nDWxyHbhqoHugYudBDKqxx",
    },
    {
      name: "token_voter",
      pid: "tokv9Lz2ZeH2F2qPcLokjoNPuvwNJ9gdZ3DaVQLPJcV",
    },
    {
      name: "nft_voter",
      pid: "nftvLQ5t6xe2nQF1NBmBBmn15ed59tU6vSCkwQNEqdc",
    },
    {
      name: "organization",
      pid: "org9nsbSiTCJzeApoS2B3uwjM2gbQH48QbUUrhAAjzG",
    },
  ]
  await Promise.all(
    programs.map(async (program) => {
      try {
        execSync(
          `anchor idl init --filepath ${__dirname}/../target/idl/${program.name}.json ${program.pid}`,
          { stdio: "inherit", shell: "/bin/bash" }
        )
      } catch {
        execSync(
          `anchor idl upgrade --filepath ${__dirname}/../target/idl/${program.name}.json ${program.pid}`,
          { stdio: "inherit", shell: "/bin/bash" }
        )
      }
    })
  )
}

export function makeid(length: number) {
  let result = ""
  const characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
  const charactersLength = characters.length
  let counter = 0
  while (counter < length) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength))
    counter += 1
  }
  return result
}
```
