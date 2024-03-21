import cp from "child_process";
import fs from "fs";
import path from "path";
// import { getOctokit, context } from "@actions/github";

const APP_VERSION = JSON.parse(
    fs.readFileSync("package.json", { encoding: "utf8" })
).version;
const TAG = `v${APP_VERSION}`;
const personal_access_token = process.env.PERSONAL_TOKEN;
const artifact_paths = process.env.ARTIFACT_PATHS;

// process.env.GH_TOKEN = "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";
// const tokenFilePath = path.join(process.env.HOME, 'gh_token.txt');
// console.log('tokenFilePath:', tokenFilePath);
// cp.execSync(`gh auth login --with-token=${tokenFilePath}`, { stdio: 'inherit' });

const tokenFilePath = path.join(process.env.HOME, "gh_token.txt");
cp.execSync(`gh auth login --with-token < ${tokenFilePath}`, {
    stdio: "inherit",
});

// const tokenFilePath = path.join(process.env.HOME, 'gh_token.txt');
// const token = fs.readFileSync(tokenFilePath, { encoding: 'utf8' }).trim();
// cp.execSync(`echo ${token} | gh auth login --with-token`, { stdio: 'inherit' });
const file = "https://github.com/TestDesktopGithubActions/test_desktop_github_actions/releases/download/v0.0.50/Falcon.Flow_0.0.50_x64-setup.nsis.zip";

const parsedPaths = JSON.parse(artifact_paths);
const artifactPaths = Array.isArray(parsedPaths) ? parsedPaths : [parsedPaths]; // 将单个路径转为数组


const quotedFilePaths = artifactPaths.map(file => `"${file}"`).join(" "); // Quote each file path and join them with a space

cp.execSync(
    `gh release create ${TAG} ${quotedFilePaths} -R https://github.com/TestDesktopGithubActions/desktop_release`,
    { stdio: "inherit" }
);

// const result = artifactPaths.join(" ");

// artifactPaths.forEach(file => {

// })



// // 获取旧仓库的release信息
// async function getOldRepoReleaseInfo() {
//     const response = await fetch(
//         "https://api.github.com/repos/TestDesktopGithubActions/test_desktop_github_actions/releases",
//         {
//             headers: {
//                 Authorization: `token ${personal_access_token}`,
//             },
//         }
//     );
//     const data = await response.json();
//     return data[0]; // 假设这里只处理最新的release
// }

// // 发布相同的release到新仓库
// async function publishToNewRepo(releaseInfo) {
//     const newReleaseUrl =
//         "https://api.github.com/repos/TestDesktopGithubActions/desktop_release/releases";
//     const response = await fetch(newReleaseUrl, {
//         method: "POST",
//         headers: {
//             Authorization: `token ${personal_access_token}`,
//             "Content-Type": "application/json",
//         },
//         body: JSON.stringify(releaseInfo),
//     });
//     if (response.status === 201) {
//         console.log("Release published to the new repository successfully");
//     } else {
//         console.error("Failed to publish release to the new repository");
//     }
// }

// // 主函数
// async function main() {
//     const oldRepoReleaseInfo = await getOldRepoReleaseInfo();
//     console.log("oldRepoReleaseInfo:", oldRepoReleaseInfo);
//     await publishToNewRepo(oldRepoReleaseInfo);
// }

// main();
