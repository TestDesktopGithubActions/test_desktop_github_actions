import cp from 'child_process';
import fs from 'fs';
import path from 'path';

const APP_VERSION = JSON.parse(fs.readFileSync('package.json', { encoding: 'utf8' })).version;
const TAG = `v${APP_VERSION}`;

console.log('env:', process.env.GH_TOKEN);
cp.execSync(`gh release create ${TAG} -R https://github.com/TestDesktopGithubActions/desktop_release`, { stdio: 'inherit' });
