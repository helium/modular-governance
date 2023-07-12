const fs = require("fs");
const path = require("path");

const packagesPath = path.join(__dirname, "..", "packages");
const docSitePath = path.join(
  __dirname,
  "..",
  "packages",
  "my-website",
  "docs",
  "API"
);

// Copy nft-voter-sdk docs -> doc-site
fs.copyFile(
  `${packagesPath}/nft-voter-sdk/docs/modules.md`,
  `${docSitePath}/nft-voter-sdk.md`,
  (err) => {
    if (err) throw err;
    console.log("nft-voter-sdk.md was copied to docsite");
  }
);

// Copy organization-sdk docs -> doc-site
fs.copyFile(
  `${packagesPath}/organization-sdk/docs/modules.md`,
  `${docSitePath}/organization-sdk.md`,
  (err) => {
    if (err) throw err;
    console.log("organization-sdk.md was copied to docsite");
  }
);

// Copy proposal-sdk docs -> doc-site
fs.copyFile(
  `${packagesPath}/proposal-sdk/docs/modules.md`,
  `${docSitePath}/proposal-sdk.md`,
  (err) => {
    if (err) throw err;
    console.log("proposal-sdk.md was copied to docsite");
  }
);

// Copy state-controller-sdk docs -> doc-site
fs.copyFile(
  `${packagesPath}/state-controller-sdk/docs/modules.md`,
  `${docSitePath}/state-controller-sdk.md`,
  (err) => {
    if (err) throw err;
    console.log("state-controller-sdk.md was copied to docsite");
  }
);

// Copy token-voter-sdk docs -> doc-site
fs.copyFile(
  `${packagesPath}/token-voter-sdk/docs/modules.md`,
  `${docSitePath}/token-voter-sdk.md`,
  (err) => {
    if (err) throw err;
    console.log("token-voter-sdk.md was copied to docsite");
  }
);
