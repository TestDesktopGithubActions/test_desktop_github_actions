import fs from "fs";
import path from "path";

async function renameFiles(file_paths, target) {
    const renamedPaths = [];
    let browser_download_url =
        "TestDesktopGithubActions/test_desktop_github_actions";
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
    console.log("browser_download_url:", browser_download_url);

    for (const filePath of file_paths) {
        if (
            filePath.includes("Falcon Flow.app.tar.gz") ||
            filePath.includes("Falcon Flow.app.tar.gz.sig")
        ) {
            const newPath = filePath.replace(
                "Falcon Flow",
                `Falcon Flow_${target}`
            );
            // const newPath = path.join(path.dirname(filePath), newFileName);
            console.error(`newPath: ${newPath}`);

            try {
                await new Promise((resolve, reject) => {
                    fs.rename(filePath, newPath, (err) => {
                        if (err) {
                            console.error(
                                `Error renaming file: ${filePath}`,
                                err
                            );
                            renamedPaths.push(filePath);
                            reject(err);
                        } else {
                            console.log(
                                `File renamed: ${filePath} -> ${newPath}`
                            );
                            renamedPaths.push(newPath);
                            resolve();
                        }
                    });
                });
            } catch (err) {
                console.error(`Error renaming file: ${filePath}`, err);
                renamedPaths.push(filePath);
            }
        } else {
            renamedPaths.push(filePath); // 不需要修改的文件直接返回
        }
    }

    return renamedPaths;
}

// 调用示例
const file_paths = ["file1/Falcon Flow.app.tar.gz"];
const target = "v1.0";
renameFiles(file_paths, target)
    .then((renamedPaths) => {
        console.log("All files renamed successfully:", renamedPaths);
    })
    .catch((error) => {
        console.error("Error occurred during file renaming:", error);
    });
