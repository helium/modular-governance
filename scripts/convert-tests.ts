const fs = require("fs");
const path = require("path");
const prettier = require("prettier");

const testsPath = path.join(__dirname, "..", "tests");
const docSitePath = path.join(
  __dirname,
  "..",
  "packages",
  "markdoc",
  "src",
  "pages",
  "docs",
  "examples"
);
const docSiteNavigationStart = path.join(
  __dirname,
  "..",
  "packages",
  "markdoc",
  "src",
  "data"
);

const convertTests = () => {
  // Get evert file in the tests folder
  const files: string[] = fs.readdirSync(testsPath);

  files.forEach((fileName) => {
    // Remove .ts from fileName
    const realFileName = fileName.split(".")[0];
    console.log(`Converting ${fileName} to ${realFileName}.md`);
    buildMd(realFileName);
    addFileToNavigation(`${realFileName}-examples`);
  });
};

const buildMd = (fileName: string) => {
  const nftVoterString: string = fs.readFileSync(
    `${testsPath}/${fileName}.ts`,
    "utf8"
  );

  const describeSplit = nftVoterString.split("describe");

  let mdFile = `
  ## Dependencies
  
  \`\`\`typescript
  ${describeSplit[0]}
  \`\`\`
  `;

  // Create a for loop that does the above for every describe and appends it to the mdFile

  for (let i = 1; i < describeSplit.length; i++) {
    const restOfFileAfterDeclarations = describeSplit[i];
    const firstLineSplit = restOfFileAfterDeclarations.split("\n");
    const firstLine = firstLineSplit[0];
    // Remove first line before \n then merge and get first example
    const firstExample = firstLineSplit.slice(1).join("\n");

    // Get first string from first line
    const firstString = firstLine.split('"')[1];

    mdFile += `
  ### ${firstString}
  
  \`\`\`typescript
  ${firstExample.split(" it(")[0]}
  \`\`\`
  `;

    // Split by it and get the rest of the examples
    const restOfExamples = restOfFileAfterDeclarations.split(" it(");

    for (let j = 1; j < restOfExamples.length; j++) {
      const example = restOfExamples[j];
      const exampleSplit = example.split("\n");
      const firstLine = exampleSplit[0];
      const firstString = firstLine.split('"')[1];
      const exampleCode = exampleSplit.slice(1).join("\n");

      mdFile += `
  #### ${firstString}
  
  \`\`\`typescript
  ${exampleCode}
  \`\`\`
  `;
    }
  }

  fs.writeFileSync(
    `${docSitePath}/${fileName}-examples.md`,
    prettier.format(mdFile, {
      semi: false,
      parser: "markdown",
    })
  );
};

const addFileToNavigation = (fileName) => {
  const navigationStart = fs.readFileSync(
    `${docSiteNavigationStart}/navigation.js`,
    "utf8"
  );

  // Find // DOCS EXAMPLES START and start adding each object with title and href
  const navigationStartSplit = navigationStart.split("// DOCS EXAMPLES START");

  const title = fileName
    .replace("-sdk", "")
    .replace(/(^|\s)\S/g, (L) => L.toUpperCase())
    .replace(/-/g, " ");

  let navigationStartSplitWithNewObject = `${navigationStartSplit[0]}// DOCS EXAMPLES START
  { title: '${title}', href: '/docs/examples/${fileName}' },\n${navigationStartSplit[1]}`;

  // Overwrite navigation.js with find and replaced string
  fs.writeFileSync(
    `${docSiteNavigationStart}/navigation.js`,
    prettier.format(navigationStartSplitWithNewObject, {
      semi: false,
      parser: "babel",
    })
  );
};

convertTests();
