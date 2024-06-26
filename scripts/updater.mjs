// scripts/updater.mjs

import fetch from "node-fetch";
import { getOctokit, context } from "@actions/github";
import fs from "fs";
// import ssh2 from "ssh2";

import updatelog from "./updatelog.mjs";

const token = process.env.GITHUB_TOKEN;
// const token = process.env.RELEASE_KEY;
const personal_access_token = process.env.PERSONAL_TOKEN;
const boss_login_body = process.env.BOSS_LOGIN_BODY;
const boss_login_url = "https://boss.ffdev.cc/v1/login";
const boss_release_add_url = "https://boss.ffdev.cc/v1/release/version";

// const serverConfig = {
//     host: "54.179.190.222",
//     port: 22,
//     username: "admin",
//     privateKey: "/home/runner/.ssh/api_id_rsa",
// };

const platformMap = {
    "darwin-aarch64": "macos",
    // "darwin": "macOS",
    "darwin-x86_64": "macos",
    "windows-x86_64": "windows",
    // "windows-x86_64-nsis": "Windows",
    "windows-x86_64-msi": "Windows",
};

const targetMap = {
    "darwin-aarch64": "aarch64",
    // "darwin": "universal",
    "darwin-x86_64": "x86_64",
    "windows-x86_64": "x86_64",
    // "windows-x86_64-nsis": "x86_64-nsis",
    // "windows-x86_64-msi": "x86_64-msi"
    "windows-x86_64-msi": "x86_64",
};

const installerMap = {
    "darwin-aarch64": "dmg",
    // "darwin": "dmg",
    "darwin-x86_64": "dmg",
    "windows-x86_64": "exe",
    // "windows-x86_64-nsis": "exe",
    "windows-x86_64-msi": "msi",
};

const pkgUrl = {
    "darwin-aarch64": "",
    "darwin-x86_64": "",
    "windows-x86_64": "",
    "windows-x86_64-msi": "",
};

// 需要生成的静态 json 文件数据，根据自己的需要进行调整
const updateData = {
    version: "",
    // 使用 UPDATE_LOG.md，如果不需要版本更新日志，则将此字段置空
    notes: "",
    pub_date: new Date().toISOString(),
    platforms: {
        // win64: { signature: '', url: '' }, // compatible with older formats
        // linux: { signature: '', url: '' }, // compatible with older formats
        // darwin: { signature: '', url: '' }, // compatible with older formats
        "darwin-aarch64": { signature: "", url: "" },
        "darwin-x86_64": { signature: "", url: "" },
        // 'linux-x86_64': { signature: '', url: '' },
        "windows-x86_64": { signature: "", url: "" },
        // 'windows-x86_64-nsis': { signature: '', url: '' },
        "windows-x86_64-msi": { signature: "", url: "" },
        // 'windows-i686': { signature: '', url: '' }, // no supported
    },
};

async function updater() {
    // const privateKeyPath = "/home/runner/.ssh/api_id_rsa";
    // const privateKeyContent = fs.readFileSync(privateKeyPath, "utf8");
    // console.log("ssh私钥: ", privateKeyContent);
    // console.log("api_private_key: ", api_private_key);
    if (!token) {
        console.log("GITHUB_TOKEN is required");
        process.exit(1);
    }

    // 用户名，仓库名
    const options = { owner: context.repo.owner, repo: context.repo.repo };
    const github = getOctokit(token);

    // 获取 tag
    const { data: tags } = await github.rest.repos.listTags({
        ...options,
        per_page: 10,
        page: 1,
    });

    // 过滤包含 `v` 版本信息的 tag
    const tag = tags.find((t) => t.name.startsWith("v"));
    // console.log(`${JSON.stringify(tag, null, 2)}`);

    if (!tag) return;

    // 获取此 tag 的详细信息
    const { data: latestRelease } = await github.rest.repos.getReleaseByTag({
        ...options,
        tag: tag.name,
    });
    console.log("latestRelease: ", latestRelease);

    updateData.version = tag.name;
    updateData.notes = updatelog(tag.name);

    //https://api.github.com/repos/TestDesktopGithubActions/test_desktop_github_actions/releases/assets/145530151
    const setSig = async (asset) => {
        // 是签名
        if (/.sig$/.test(asset.name)) {
            let sig = await _getSignature(asset.url);
            // let sig = await getSignature(asset.browser_download_url);
            console.log("[setSig] get sig: ", sig);
            // macos
            if (/_aarch64.app.tar.gz.sig$/.test(asset.name)) {
                updateData.platforms["darwin-aarch64"].signature = sig;
            }
            // else if (/_universal.app.tar.gz.sig$/.test(asset.name)) {
            //   updateData.platforms['darwin'].signature = sig;
            // }
            else if (/_x64.app.tar.gz.sig$/.test(asset.name)) {
                updateData.platforms["darwin-x86_64"].signature = sig;
            }
            // windows
            else if (/_x64-setup.nsis.zip.sig$/.test(asset.name)) {
                // updateData.platforms['windows-x86_64-nsis'].signature = sig;
                updateData.platforms["windows-x86_64"].signature = sig;
            } else if (/_x64_en-US.msi.zip.sig$/.test(asset.name)) {
                updateData.platforms["windows-x86_64-msi"].signature = sig;
            }
        }
    };

    const setUrl = async (asset) => {
        let browser_download_url = asset.browser_download_url;
        if (
            browser_download_url.includes(
                "TestDesktopGithubActions/test_desktop_github_actions"
            )
        ) {
            browser_download_url = browser_download_url.replace(
                "TestDesktopGithubActions/test_desktop_github_actions",
                `TestDesktopGithubActions/desktop_release`
            );
        }
        // 设置下载链接
        // macos
        if (/_aarch64.app.tar.gz$/.test(asset.name)) {
            updateData.platforms["darwin-aarch64"].url = browser_download_url;
            pkgUrl["darwin-aarch64"] = browser_download_url;
        }
        // else if (/_universal.app.tar.gz$/.test(asset.name)) {
        //   updateData.platforms['darwin'].url = asset.browser_download_url;
        // }
        else if (/_x64.app.tar.gz$/.test(asset.name)) {
            updateData.platforms["darwin-x86_64"].url = browser_download_url;
            pkgUrl["darwin-x86_64"] = browser_download_url;
        }
        // windows
        else if (/_x64-setup.nsis.zip$/.test(asset.name)) {
            updateData.platforms["windows-x86_64"].url = browser_download_url;
            pkgUrl["windows-x86_64"] = browser_download_url;
            // updateData.platforms['windows-x86_64-nsis'].url = asset.browser_download_url;
        } else if (/_x64_en-US.msi.zip$/.test(asset.name)) {
            updateData.platforms["windows-x86_64-msi"].url =
                browser_download_url;
            pkgUrl["windows-x86_64-msi"] = browser_download_url;
        }
        console.log("[setUrl] updateData: ", updateData);
    };

    const setPkgUrl = async (asset) => {
        let remoteFilePath = "";
        if (/_aarch64.dmg$/.test(asset.name)) {
            remoteFilePath = `/opt/www/rf/api/releases/macos/aarch64/dmg/${asset.name}`;
            // pkgUrl["darwin-aarch64"] = asset.browser_download_url;
        } else if (/_x64.dmg$/.test(asset.name)) {
            remoteFilePath = `/opt/www/rf/api/releases/macos/x86_64/dmg/${asset.name}`;
            // pkgUrl["darwin-x86_64"] = asset.browser_download_url;
        }
        // windows
        else if (/_x64_en-US.msi$/.test(asset.name)) {
            remoteFilePath = `/opt/www/rf/api/releases/windows/x86_64/msi/${asset.name}`;
            // pkgUrl["windows-x86_64-msi"] = asset.browser_download_url;
        } else if (/_x64-setup.exe$/.test(asset.name)) {
            remoteFilePath = `/opt/www/rf/api/releases/windows/x86_64/nsis/${asset.name}`;
            // pkgUrl["windows-x86_64"] = asset.browser_download_url;
        }
        // uploadGitHubFileToServer(asset.url, remoteFilePath, serverConfig);
    };

    const setAsset = async (asset) => {
        console.log("[setAsset] asset: ", asset);
        console.log("[setAsset] name: ", asset.name);
        console.log(
            "[setAsset] browser_download_url: ",
            asset.browser_download_url
        );
        // console.log('[setAsset] reg: ', reg);
        await setSig(asset);
        await setUrl(asset);
        await setPkgUrl(asset);
    };

    const promises = latestRelease.assets.map(async (asset) => {
        await setAsset(asset);
    });

    await Promise.allSettled(promises);

    if (!fs.existsSync("updater")) {
        fs.mkdirSync("updater");
    }

    // 将数据写入文件
    fs.writeFileSync(
        "./updater/install.json",
        JSON.stringify(updateData, null, 2)
    );
    console.log("Generate updater/install.json");
}

async function exec() {
    try {
        let boss_login_token = await getBossToken();
        await updater();
        await addPackageVersion(boss_login_token);
    } catch (error) {
        console.error(error);
    }
    // updater().catch(console.error);
    // addPackageVersion().catch(console.error);
}

exec().catch(console.error);

// 获取token
async function getBossToken() {
    try {
        let json_str = JSON.parse(boss_login_body);
        const response = await fetch(boss_login_url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(json_str),
        });

        if (response.ok) {
            const { code, message, result } = await response.json();

            if (code != 200) {
                console.error("Failed to get boss token:", message);
            } else {
                const { token } = result;
                console.log("Get boss token successfully");
                return token;
            }
        } else {
            console.error("Failed to get boss token:", response.statusText);
            return "";
        }
    } catch (error) {
        console.error("Error fetching or parsing data:", error);
        return "";
    }
}

// 获取签名内容
async function _getSignature(url) {
    try {
        const response = await fetch(url, {
            method: "GET",
            headers: {
                Accept: "application/octet-stream",
                Authorization: `token ${personal_access_token}`,
            },
        });
        return response.text();
    } catch (_) {
        return "";
    }
}

// 封装发送POST请求的函数
async function addPackageVersion(boss_login_token) {
    for (const [platform, data] of Object.entries(updateData.platforms)) {
        console.log(`[${platform}] Start add package version!`);
        console.log(`[${platform}] download url: ${data.url}`);
        const packageData = {
            platform: platformMap[platform] || "Unknown",
            version: updateData.version,
            target: targetMap[platform] || "Unknown",
            installer: installerMap[platform] || "Unknown",
            notes: updateData.notes,
            // download_url: data.url,
            download_url: pkgUrl[platform],
        };

        console.log(
            `[${platform}] packageData: ${JSON.stringify(packageData)}`
        );
        const requestOptions = {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: boss_login_token,
            },
            body: JSON.stringify(packageData),
        };

        try {
            const response = await fetch(boss_release_add_url, requestOptions);
            if (response.ok) {
                const { code, message } = await response.json();

                console.log("code:", code);
                if (code != 200) {
                    console.error(
                        "Failed to add package version information:",
                        message
                    );
                } else {
                    console.log(
                        `[${platform} ${data.target} ${data.installer}] Added package version information successfully!`
                    );
                }
            } else {
                console.error(
                    "Failed to add package version information:",
                    response.statusText
                );
            }
        } catch (error) {
            console.error("An error has occurred:", error);
        }
    }
}

// // 上传文件到api server
// async function uploadGitHubFileToServer(url, remoteFilePath, serverConfig) {
//     try {
//         const response = await fetch(url, {
//             method: "GET",
//             headers: {
//                 Accept: "application/octet-stream",
//                 Authorization: `token ${personal_access_token}`,
//             },
//         });

//         const fileData = await response.buffer();
//         await UploadPackage(fileData, remoteFilePath, serverConfig);
//     } catch (error) {
//         console.error("Error:", error);
//         return error;
//     }
// }

// // 封装发送POST请求的函数
// async function UploadPackage(fileData, remoteFilePath, serverConfig) {
//     return new Promise((resolve, reject) => {
//         const conn = new ssh2.Client();

//         conn.on("ready", function () {
//             console.log("SSH connection established");

//             conn.sftp(function (err, sftp) {
//                 if (err) {
//                     conn.end();
//                     reject(err);
//                 }

//                 const writeStream = sftp.createWriteStream(remoteFilePath);
//                 writeStream.on("close", function () {
//                     console.log("File transferred");
//                     conn.end();
//                     resolve();
//                 });
//                 writeStream.on("error", function (err) {
//                     console.error("File transfer error:", err);
//                     conn.end();
//                     reject(err);
//                 });

//                 writeStream.write(fileData);
//                 writeStream.end();
//             });
//         });
//         conn.on("error", function (err) {
//             console.error("SSH connection error:", err);
//             conn.end();
//             reject(err);
//         });

//         conn.connect(serverConfig);
//     });
// }
