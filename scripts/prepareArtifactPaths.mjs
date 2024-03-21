const fs = require('fs');
const path = require('os').homedir() + '/new/artifact/file.txt'; // 生成文件路径

const paths = process.env.ARTIFACT_PATHS;
const parsedPaths = JSON.parse(paths);
const artifactPaths = Array.isArray(parsedPaths) ? parsedPaths : [parsedPaths]; // 将单个路径转为数组
// console.log(artifactPaths.join("\n"));
const result = artifactPaths.join("\n");

// 创建目录以及写入文件
fs.mkdirSync(path.split('/').slice(0, -1).join('/'), { recursive: true }); // 递归创建目录
fs.writeFileSync(path, result); // 将 result 写入文件

console.log("NEW_ARTIFACT_PATHS: ",process.env.NEW_ARTIFACT_PATHS);
console.log(result); // 输出到控制台
export default result;