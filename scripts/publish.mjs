import cp from 'child_process';
import fs from 'fs';
import path from 'path';

const APP_VERSION = JSON.parse(fs.readFileSync('package.json', { encoding: 'utf8' })).version;
const TAG = `v${APP_VERSION}`;

process.env.GH_TOKEN = "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";
cp.execSync(`gh release create ${TAG} -R https://github.com/TestDesktopGithubActions/desktop_release`, { stdio: 'inherit' });
