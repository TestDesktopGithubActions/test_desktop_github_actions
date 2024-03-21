const paths = process.env.ARTIFACT_PATHS;
const parsedPaths = JSON.parse(paths);
const artifactPaths = Array.isArray(parsedPaths) ? parsedPaths : [parsedPaths]; // 将单个路径转为数组
// console.log(artifactPaths.join("\n"));
const result = artifactPaths.join("\n");
console.log(result); // 输出到控制台
export default result;