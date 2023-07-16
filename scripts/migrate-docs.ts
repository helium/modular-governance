const fs = require("fs");
const path = require("path");
const prettier = require("prettier");

const packagesPath = path.join(__dirname, "..", "packages");
const docSitePath = path.join(
  __dirname,
  "..",
  "packages",
  "markdoc",
  "src",
  "pages",
  "docs",
  "api"
);
const docSiteNavigationStart = path.join(
  __dirname,
  "..",
  "packages",
  "markdoc",
  "src",
  "data"
);

const migrateDocs = () => {
  clearNavigation();

  // Get every file in the packages folder
  const files: string[] = fs.readdirSync(packagesPath);

  files.forEach((fileName) => {
    try {
      migrateTypedocToDocs(fileName);
      addFileToNavigation(fileName);
    } catch (e) {
      console.log(`Docs for ${fileName} not found`);
    }
  });
};

const clearNavigation = () => {
  const navigationStart = fs.readFileSync(
    `${docSiteNavigationStart}/navigation.js`,
    "utf8"
  );
  // First clear everthing in between // Find // DOCS NAVIGATION START and // DOCS NAVIGATION END
  const navigationStartSplitEnd = navigationStart.split(
    "// DOCS NAVIGATION END"
  );

  const navStart = navigationStartSplitEnd[0].split("// DOCS NAVIGATION START");

  let navigationStartSplitEndWithNewObject = `${navStart[0]}
      // DOCS NAVIGATION START
      // DOCS NAVIGATION END${navigationStartSplitEnd[1]}`;

  // Overwrite navigation.js with find and replaced string
  fs.writeFileSync(
    `${docSiteNavigationStart}/navigation.js`,
    prettier.format(navigationStartSplitEndWithNewObject, {
      semi: false,
      parser: "babel",
    })
  );
};

const addFileToNavigation = (fileName) => {
  const navigationStart = fs.readFileSync(
    `${docSiteNavigationStart}/navigation.js`,
    "utf8"
  );

  // Find // DOCS NAVIGATION START and start adding each object with title and href
  const navigationStartSplit = navigationStart.split(
    "// DOCS NAVIGATION START"
  );

  const title = fileName
    .replace("-sdk", "")
    .replace(/(^|\s)\S/g, (L) => L.toUpperCase())
    .replace(/-/g, " ");

  let navigationStartSplitWithNewObject = `${navigationStartSplit[0]}// DOCS NAVIGATION START
{ title: '${title}', href: '/docs/api/${fileName}' },\n${navigationStartSplit[1]}`;

  // Overwrite navigation.js with find and replaced string
  fs.writeFileSync(
    `${docSiteNavigationStart}/navigation.js`,
    prettier.format(navigationStartSplitWithNewObject, {
      semi: false,
      parser: "babel",
    })
  );
};

const migrateTypedocToDocs = (fileName) => {
  const modulesMd: string = fs.readFileSync(
    `${packagesPath}/${fileName}/docs/modules.md`,
    "utf8"
  );

  // Find and replace modules.md with filename and remove .md
  const modulesMdReplaced = modulesMd
    .replace(/modules.md/g, `${fileName}`)
    .replace(/.md/g, "");

  // Create a regex that matches the above strings and finds with filename then replaces the string after # with the string inside the brackets
  const regex = new RegExp(`- \\[(.*)\\]\\(${fileName}#(.*)\\)`, "g");

  // Convert $1 to - per every camelcase
  const modulesMdReplacedWithLinks = modulesMdReplaced.replace(
    regex,
    (_, p1, p2) => {
      const camelCaseToDash = p1
        .replace(/([a-z])([A-Z])/g, "$1-$2")
        .toLowerCase();
      return `- [${p1}](${fileName}#${camelCaseToDash})`;
    }
  );

  // Overwrite modules.md with find and replaced string
  fs.writeFileSync(
    `${packagesPath}/${fileName}/docs/modules.md`,
    modulesMdReplacedWithLinks
  );

  // Copy filename docs -> doc-site
  fs.copyFile(
    `${packagesPath}/${fileName}/docs/modules.md`,
    `${docSitePath}/${fileName}.md`,
    (err) => {
      if (err) throw err;
      console.log(`${fileName}.md was copied to docsite`);
    }
  );
};

migrateDocs();
