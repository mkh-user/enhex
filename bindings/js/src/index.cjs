module.exports = {
    get compile() {
        return require('../pkg/enhex-core.cjs').compile;
    },
    get compileRegExp() {
        const { compile } = require('../pkg/enhex-core.cjs');
        return (pattern, flags = '') => new RegExp(compile(pattern), flags);
    },
    get enhex() {
        const { compile } = require('../pkg/enhex-core.cjs')
        return (strings, ...values) => {
            let pattern = '';
            for (let i = 0; i < strings.length; i++) {
                pattern += strings[i];
                if (i < values.length) {
                    pattern += values[i];
                }
            }
            return compile(pattern);
        }
    }
};
