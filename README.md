# MikuModManager
- 在安装游戏模组或补丁时，您有没有遇到游戏无法打开，效果不喜欢，但是缺少恢复的手段？因为您的补丁文件已经嵌入了游戏文件中？
- 使用MikuModManager可以安全且简单的安装模组，且在您的游戏遇到问题需要恢复时一键卸载回到安装前状态！
- 添加游戏会自动创建对应文件夹结构，支持模组管理、冲突检测、自动备份原文件
- 将您要管理的模组放入`game/[游戏名称]/mods/`目录下即可被程序识别到

## 后续更新
- 集成模组版本管理，如果有更新的版本则提醒
- 

## 使用方法

### 添加游戏
1. 点击主界面上的「添加游戏」按钮
2. 输入游戏名称并选择游戏安装目录
3. （可选）绑定MikuGame数据库中的游戏信息
4. 点击「添加游戏」按钮完成添加
5. 系统会自动创建以下目录结构：
   ```
   game/[游戏名称]/
   ├── mods/           # 存放模组
   ├── backup/         # 自动备份的原始文件
   ├── installLog/     # 模组安装日志
   └── game_status.json # 游戏状态文件
   ```

### 安装模组
1. 将模组文件夹放置在 `game/[游戏名称]/mods/` 目录下
2. 模组结构应为：
   ```
   mods/
   └── {文件夹名}/        # 文件夹名就是模组ID
       ├── mod.json       # 模组配置文件
       └── mod/           # 模组实际文件
   ```
3. 在游戏详情页面的「模组管理」模块中，启用/禁用模组
4. 点击「应用更改」按钮安装或卸载模组

### 卸载模组
1. 在游戏详情页面中，取消勾选要卸载的模组
2. 点击「应用更改」按钮
3. 系统会自动恢复原始文件

## 联系我

- 项目负责人: [shshouse](https://github.com/shshouse)
- Bilibili: [shshouse](https://space.bilibili.com/3493127123897196)
- 爱发电: [shshouse](https://afdian.com/a/shshouse)
- 反馈问题: [Issues](https://github.com/shshouse/MikuModsManager/issues)

## 鸣谢

- [Vue.js](https://vuejs.org/)
- [TypeScript](https://www.typescriptlang.org/)
- [Tauri](https://tauri.app/)

---

如果您觉得这玩意有用，请给一个star支持一下qwq
