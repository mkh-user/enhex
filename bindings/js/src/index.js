import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const core = require('../pkg/enhex-core.cjs');

/**
 * Compile an EnhEx pattern string to a Regex string (synchronous).
 *
 * @param {string} pattern - The EnhEx pattern to compile.
 * @returns {string} The compiled Regex string.
 *
 * @example
 * const regex = compile("start + one_or_more(digit) + end");
 * // regex = "^\\d+$"
 */
export function compile(pattern) {
    return core.compile(pattern);
}

/**
 * Compile an EnhEx pattern and return a RegExp object directly.
 *
 * @param {string} pattern - The EnhEx pattern to compile.
 * @param {string} [flags] - RegExp flags (e.g., "g", "i", "m").
 * @returns {RegExp}
 */
export function compileRegExp(pattern, flags = '') {
    return new RegExp(compile(pattern), flags);
}

/**
 * Tagged template literal for EnhEx patterns.
 *
 * @param {string[]} strings - Template string parts
 * @param {...any} values - Interpolated values
 * @returns {string} Compiled Regex string
 *
 * @example
 * const regex = enhex`start + one_or_more(digit) + end`;
 * // regex = "^\\d+$"
 *
 * @example
 * const domain = "example\\.com";
 * const regex = enhex`start + one_or_more(word_char | dot | dash) + "@" + ${domain} + end`;
 */
export function enhex(strings, ...values) {
    // Combine template parts with interpolated values
    let pattern = '';
    for (let i = 0; i < strings.length; i++) {
        pattern += strings[i];
        if (i < values.length) {
            pattern += values[i];
        }
    }

    return compile(pattern);
}
