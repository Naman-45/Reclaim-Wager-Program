const path = require("path");
const programDir = path.join(__dirname, "..", "programs/reclaim");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "reclaim",
  programId: "HN29XytrchMhZHw3r6og13SQLZAxMvSm9yQJkuw2x3GK",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
