const {pick, pipe} = require('./common');
const toml = require('@iarna/toml');

/// # Public
module.exports = {
    readVersion: cargoTomlContents => getVer(cargoTomlContents),
    writeVersion: (cargoTomlContents, version) => setVer(cargoTomlContents, version),
};

/// # Not Public

// read the "version" property in Cargo.toml
const getVer = cargoToml => toml.parse(cargoToml).package.version;

// update the "version" property with a new version
const setVer = (cargoToml, newVersion) => pipe( toml.parse
                                              , cargo => {
                                                  cargo.package.version = newVersion;

                                                  // HACK: if deps contains happi-derive, update it.
                                                  if (cargo.dependencies && cargo.dependencies['happi-derive']) {
                                                    cargo.dependencies['happi-derive'].version = newVersion;
                                                  }

                                                  return cargo;
                                                }
                                              , toml.stringify
                                              )
                                              (cargoToml);

/// # Tests
const test = `
[package]
version = "0.4.0"

[dependencies]
happi-derive = "0.4.0"
`;

if (getVer(test) !== '0.4.0') {
    throw new Error('in ' + toml + ' expected 0.4.0 got ' + getVer(toml));
}

let updated = setVer(test, "0.5.0");
if (getVer(updated) !== '0.5.0') {
    throw new Error('in ' + toml + ' expected 0.5.0 got ' + getVer(toml));
}
