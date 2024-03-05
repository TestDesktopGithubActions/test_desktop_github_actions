const paths = process.env.ARTIFACT_PATHS;
const parsedPaths = JSON.parse(paths);
const artifactPaths = Array.isArray(parsedPaths) ? parsedPaths : [parsedPaths]; // 将单个路径转为数组
// console.log(artifactPaths.join("\n"));
process.stdout.write(artifactPaths.join("\n"));