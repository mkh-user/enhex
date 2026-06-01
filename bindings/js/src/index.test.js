import { compile, compileRegExp, enhex } from './index.js';
import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

describe('EnhEx JavaScript Binding', () => {
    it('should compile digits', () => {
        assert.equal(compile('start + one_or_more(digit) + end'), '^\\d+$');
    });

    it('should compile email', () => {
        assert.equal(
            compile('start + one_or_more(word_char | dot | dash) + "@" + one_or_more(word_char | dash) + "." + tld() + end'),
            '^[\\w\\.-]+@[\\w-]+\\.[a-z]{2,10}$'
        );
    });

    it('should compile lazy quantifier', () => {
        assert.equal(compile('start + zero_or_more_lazy(anything) + end'), '^.*?$');
    });

    it('should compile negated class', () => {
        assert.equal(compile('start + one_or_more(not(digit)) + end'), '^[^\\d]+$');
    });

    it('should compile backreference', () => {
        assert.equal(
            compile('start + group(one_or_more(word_char)) + " " + backref(1) + end'),
            '^(\\w+) \\1$'
        );
    });

    it('should compile hex digit', () => {
        assert.equal(compile('start + exactly(6, hex_digit) + end'), '^[\\da-fA-F]{6}$');
    });

    it('should compile preset', () => {
        assert.equal(compile('start + ipv4() + end'), '^(?:\\d{1,3}\\.){3}\\d{1,3}$');
    });

    it('compileRegExp should return RegExp', () => {
        const re = compileRegExp('start + one_or_more(digit) + end');
        assert.ok(re.test('12345'));
        assert.ok(!re.test('abc'));
    });

    it('compileRegExp with flags', () => {
        const re = compileRegExp('start + one_or_more(letter) + end', 'gi');
        assert.ok(re.global);
        assert.ok(re.ignoreCase);
        assert.ok(re.test('abc'));
    });

    it('should work as tagged template literal', () => {
        const regex = enhex`start + one_or_more(digit) + end`;
        assert.equal(regex, '^\\d+$');
    });

    it('should support interpolation in tagged template', () => {
        const subPattern = 'one_or_more(digit)';
        const regex = enhex`start + ${subPattern} + end`;
        assert.equal(regex, '^\\d+$');
    });
});
