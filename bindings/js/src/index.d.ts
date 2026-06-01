/**
 * Compile an EnhEx pattern string to a Regex string.
 * @param pattern - The EnhEx pattern to compile.
 * @returns The compiled Regex string.
 */
export function compile(pattern: string): string;

/**
 * Compile an EnhEx pattern and return a RegExp object directly.
 * @param pattern - The EnhEx pattern to compile.
 * @param flags - RegExp flags (e.g., "g", "i", "m").
 * @returns A RegExp object.
 */
export function compileRegExp(pattern: string, flags?: string): RegExp;

/**
 * Tagged template literal for EnhEx patterns.
 * @param strings - Template string parts
 * @param values - Interpolated values
 * @returns Compiled Regex string
 */
export function enhex(strings: TemplateStringsArray, ...values: any[]): string;
