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
