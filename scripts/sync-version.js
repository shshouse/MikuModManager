import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// 获取当前文件的目录路径
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 读取版本配置文件
const versionConfigPath = path.join(__dirname, '..', 'version.json');
let versionData;

try {
  const versionConfig = fs.readFileSync(versionConfigPath, 'utf8');
  versionData = JSON.parse(versionConfig);
} catch (error) {
  console.error('无法读取version.json文件:', error);
  process.exit(1);
}

const version = versionData.version;

if (!version) {
  console.error('version.json中没有version字段');
  process.exit(1);
}

console.log(`正在同步版本号: ${version}`);

// 同步package.json
const packageJsonPath = path.join(__dirname, '..', 'package.json');
try {
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  packageJson.version = version;
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2), 'utf8');
  console.log('✓ package.json 已更新');
} catch (error) {
  console.error('更新package.json失败:', error);
}

// 同步tauri.conf.json
const tauriConfPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');
try {
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
  tauriConf.version = version;
  fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2), 'utf8');
  console.log('✓ tauri.conf.json 已更新');
} catch (error) {
  console.error('更新tauri.conf.json失败:', error);
}

// 创建版本模块文件
try {
  const versionModulePath = path.join(__dirname, '..', 'src', 'version.ts');
  const versionModuleContent = `// 全局版本信息 - 由sync-version.js自动生成
// 请勿手动修改此文件
export const APP_VERSION = '${version}';
`;
  fs.writeFileSync(versionModulePath, versionModuleContent, 'utf8');
  console.log('✓ src/version.ts 已创建/更新');
} catch (error) {
  console.error('创建版本模块文件失败:', error);
}

console.log('版本同步完成！');