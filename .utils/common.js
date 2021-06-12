// returns a bool indicating whether a given val is null or undefined
const isNullish = val => typeof val === 'null' || typeof val === 'undefined';

// safely access a property
const pick = (val, prop) => isNullish(val) ? undefined : val[prop];

const pipe = (...fs) => v => fs.reduce((v_, f) => f(v_), v);

module.exports = {isNullish, pick, pipe};
