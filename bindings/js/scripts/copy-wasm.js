import { copyFileSync, mkdirSync, existsSync, renameSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, '..');

const sourceDir = join(rootDir, '..', '..', 'core', 'pkg');
const destDir = join(rootDir, 'pkg');

if (!existsSync(destDir)) {
    mkdirSync(destDir, { recursive: true });
}

const files = [
    'enhex-core_bg.wasm',
    'enhex-core.js',
    'enhex-core.d.ts',
    'enhex-core_bg.wasm.d.ts',
];

for (const file of files) {
    const src = join(sourceDir, file);
    const dest = join(destDir, file);
    if (existsSync(src)) {
        copyFileSync(src, dest);
    } else {
        console.warn(`Warning: ${file} not found in ${sourceDir}`);
    }
}

const oldPath = join(destDir, 'enhex-core.js');
const newPath = join(destDir, 'enhex-core.cjs');
if (existsSync(oldPath)) {
    renameSync(oldPath, newPath);
}

console.log('WASM files copied successfully.');

const meta_files = [
    'LICENSE',
    'README.md',
]

for (const file of meta_files) {
    const src = join(rootDir, '..', '..', file);
    const dest = join(rootDir, file);
    if (existsSync(join(src))) {
        copyFileSync(src, dest);
    } else {
        console.warn(`Warning: ${file} not found in ${join(rootDir, '..', '..')}`);
    }
}

console.log('Meta files copied successfully.')
