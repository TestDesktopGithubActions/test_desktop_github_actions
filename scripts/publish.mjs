import cp from 'child_process';
import fs from 'fs';
import path from 'path';

const APP_VERSION = JSON.parse(fs.readFileSync('package.json', { encoding: 'utf8' })).version;
const TAG = `v${APP_VERSION}`;

// process.env.GH_TOKEN = "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";
const tokenFilePath = path.join(process.env.HOME, 'gh_token.txt');
cp.execSync(`gh auth login --with-token=$(cat ${tokenFilePath})`, { stdio: 'inherit' });

cp.execSync(`gh release create ${TAG} -R https://github.com/TestDesktopGithubActions/desktop_release`, { stdio: 'inherit' });
