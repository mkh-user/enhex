// CommonJS wrapper for require() users
module.exports = {
    get compile() {
        return require('../pkg/enhex-core.cjs').compile;
    },
    get compileRegExp() {
        // Use the ESM compile function for consistency
        const { compile } = require('../pkg/enhex-core.cjs');
        return (pattern, flags = '') => new RegExp(compile(pattern), flags);
    },
};
