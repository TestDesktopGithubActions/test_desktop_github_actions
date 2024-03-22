import fs from "fs";
import path from "path";

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

// let boss_login_token =
//     "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI1NjBhMzkyNTgxYmUxYzQ0YzI3YjNhZmY0NjI4ZjdkZSIsImtleSI6WzcsOTAsMTUzLDU5LDIyNSwxMzcsMjIxLDg0LDc3LDE4Nyw1MSwyOSwxOTMsMTA5LDkwLDY0LDg2LDExOCw5MSw1OSwxNTcsMjQxLDk4LDI3LDEwOSw1MCwxODksMTA0LDczLDE1LDEyMywxMjJdLCJpYXQiOjE3MTExMTg4NDQsImV4cCI6MTcxMjQxNDg0NH0.nB3bKmS263qcc3BHOI_DPo8PLAAQhb1fD8-I0hjHZjg";
const boss_release_add_url = "https://boss.ffdev.cc/v1/release/version";
const boss_login_url = "https://boss.ffdev.cc/v1/login";

// let boss_login_body = {
//     user_name: "x",
//     password: "430c939a46035151f598ee338d4449d6",
// };

let boss_login_body =
    '{"user_name":"x","password":"430c939a46035151f598ee338d4449d6"}';
// let json = JSON.stringify(boss_login_body);
let json_str = JSON.parse(boss_login_body);
let json = JSON.stringify(json_str);


// 获取token
async function getBossToken() {
    try {
        console.log("json: ",json);
        const response = await fetch(boss_login_url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: json,
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

        console.log(`[${platform}] packageData: ${packageData}`);
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

async function exec() {
    try {
        let boss_login_token = await getBossToken();
        await addPackageVersion(boss_login_token);
    } catch (error) {
        console.error(error);
    }
    // updater().catch(console.error);
    // addPackageVersion().catch(console.error);
}

exec().catch(console.error);

// async function renameFiles(file_paths, target) {
//     const renamedPaths = [];
//     let browser_download_url =
//         "TestDesktopGithubActions/test_desktop_github_actions";
//     if (
//         browser_download_url.includes(
//             "TestDesktopGithubActions/test_desktop_github_actions"
//         )
//     ) {
//         browser_download_url = browser_download_url.replace(
//             "TestDesktopGithubActions/test_desktop_github_actions",
//             `TestDesktopGithubActions/desktop_release`
//         );
//     }
//     console.log("browser_download_url:", browser_download_url);

//     for (const filePath of file_paths) {
//         if (
//             filePath.includes("Falcon Flow.app.tar.gz") ||
//             filePath.includes("Falcon Flow.app.tar.gz.sig")
//         ) {
//             const newPath = filePath.replace(
//                 "Falcon Flow",
//                 `Falcon Flow_${target}`
//             );
//             // const newPath = path.join(path.dirname(filePath), newFileName);
//             console.error(`newPath: ${newPath}`);

//             try {
//                 await new Promise((resolve, reject) => {
//                     fs.rename(filePath, newPath, (err) => {
//                         if (err) {
//                             console.error(
//                                 `Error renaming file: ${filePath}`,
//                                 err
//                             );
//                             renamedPaths.push(filePath);
//                             reject(err);
//                         } else {
//                             console.log(
//                                 `File renamed: ${filePath} -> ${newPath}`
//                             );
//                             renamedPaths.push(newPath);
//                             resolve();
//                         }
//                     });
//                 });
//             } catch (err) {
//                 console.error(`Error renaming file: ${filePath}`, err);
//                 renamedPaths.push(filePath);
//             }
//         } else {
//             renamedPaths.push(filePath); // 不需要修改的文件直接返回
//         }
//     }

//     return renamedPaths;
// }

// // 调用示例
// const file_paths = ["file1/Falcon Flow.app.tar.gz"];
// const target = "v1.0";
// renameFiles(file_paths, target)
//     .then((renamedPaths) => {
//         console.log("All files renamed successfully:", renamedPaths);
//     })
//     .catch((error) => {
//         console.error("Error occurred during file renaming:", error);
//     });
