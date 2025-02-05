const path = require("path");
const programDir = path.join(__dirname, "..", "programs/reclaim");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "reclaim",
  programId: "CGTjkfCkFqEPhp28aBK6afd2SaqeVTju1pdYZzdrX3dn",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
