import fs from 'node:fs';
import path, { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

interface Command {
    methodName: string;
    parameters: string[];
    returnValue: string;
}

function extractCommandsFromRustFile(filePath: string) {
    const fileContent = fs.readFileSync(filePath, 'utf-8');
    const regex = /#[\s]*\[tauri::command\][\s]*pub async fn ([a-zA-Z_][a-zA-Z0-9_]*)\(([^)]*)\)(?: -> ([^{]*))? {/g;

    const commands: Command[] = [];
    const matches = fileContent.matchAll(regex);

    for (const match of matches) {
        const methodName = match[1];
        const parameters = match[2].split(',').map(param => param.trim());
        const returnValue = match[3] ? match[3].trim() : 'void';

        commands.push({
            methodName,
            parameters,
            returnValue,
        });
    }

    return commands;
}

function getAllActionRsFiles(directoryPath) {
    const allFiles: string[] = [];

    function processDirectory(dirPath) {
        const files = fs.readdirSync(dirPath);

        for (const file of files) {
            const filePath = path.join(dirPath, file);
            const stat = fs.statSync(filePath);

            if (stat.isDirectory()) {
                // 如果是子目录，递归扫描
                processDirectory(filePath);
            } else if (
                path.extname(file) === '.rs'
                && file === 'action.rs' // 检查文件名是否为 action.rs
            ) {
                allFiles.push(filePath);
            }
        }
    }

    processDirectory(directoryPath);
    return allFiles;
}

// Specify the path to the directory containing Rust source files
const rustDirectoryPath = path.join(__dirname, '../src-tauri', 'src', 'service');

// 获取所有文件名为 `action.rs` 的文件的路径，并提取命令
const actionRsFiles = getAllActionRsFiles(rustDirectoryPath);

// 提取命令
const commandList: Command[] = [];

for (const actionRsFile of actionRsFiles) {
    const commands = extractCommandsFromRustFile(actionRsFile);
    commandList.push(...commands);
}

// console.log(commandList);

// 生成 TypeScript 类型定义文件
function generateTypeDefinitions(commandList: Command[]) {
    let typeDefinitions = '';

    for (const command of commandList) {
        const { methodName, parameters, returnValue } = command;

        // Extract parameter types and names
        const parsedParameters = parameters
            .filter(param => param.trim() !== '')
            .filter(param => !param.includes('tauri::'))
            .map((param) => {
                const [name, type] = param.split(':');
                return { name: convertToCamelCase(name.trim()), type: type ? type.trim() : 'any' };
            });

        // Generate command input type definition
        const inputTypeDefinition = parsedParameters.length
            ? `type ${capitalizeAndRemoveUnderscores(methodName)}Input = { ${
                parsedParameters.map(param => `${param.name}: ${param.type}`).join(', ')
            } };`
            : `type ${capitalizeAndRemoveUnderscores(methodName)}Input = {};`;

        // Generate command output type definition
        const outputTypeDefinition = `type ${capitalizeAndRemoveUnderscores(methodName)}Output = ${JSON.stringify(returnValue)};`;

        // Generate command type definition
        const commandTypeDefinition = `type ${capitalizeAndRemoveUnderscores(methodName)}Command = { name: '${methodName}', input: ${capitalizeAndRemoveUnderscores(methodName)}Input, output: ${capitalizeAndRemoveUnderscores(methodName)}Output };`;

        // Concatenate type definitions
        typeDefinitions += `${inputTypeDefinition}\n${outputTypeDefinition}\n${commandTypeDefinition}\n\n`;
    }

    // Generate type Command
    typeDefinitions += `type Command = ${
        commandList.map(({ methodName }) => `${capitalizeAndRemoveUnderscores(methodName)}Command`).join(' | ')
    };`;

    // Write type definitions to file
    fs.writeFileSync('tauri-command.d.ts', typeDefinitions);
}

function capitalizeAndRemoveUnderscores(str: string): string {
    return str
        .replace(/^_/, '')
        .split('_')
        .map((word, index) => index === 0 ? word.charAt(0).toUpperCase() + word.slice(1) : word.charAt(0).toUpperCase() + word.slice(1))
        .join('');
}

function convertToCamelCase(input: string): string {
    return input
        .replace(/^_/, '')
        .split('_')
        .map((word, index) => index === 0 ? word : word.charAt(0).toUpperCase() + word.slice(1))
        .join('');
}

// 调用生成类型定义的函数
generateTypeDefinitions(commandList);

console.log('> Type definitions generated successfully!');
