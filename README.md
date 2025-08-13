# MikuModsManager

![MikuModsManager Logo](public/MikuMods.png)

MikuModsManager 是一个基于 Vue 3、TypeScript 和 Tauri 的桌面端应用程序，用于管理和安装游戏补丁。它提供了一个直观的用户界面，让玩家可以轻松地安装、管理和回滚游戏补丁。

## ✨ 功能特点

- **游戏管理**：添加和管理多个游戏
- **补丁安装**：轻松安装和管理游戏补丁
- **备份回滚**：自动创建补丁安装备份，支持一键回滚
- **用户友好界面**：简洁直观的界面设计，易于使用
- **跨平台支持**：基于 Tauri 构建，支持 Windows、macOS 和 Linux

## 🚀 安装指南

### 前提条件

- [Node.js](https://nodejs.org/) (v16 或更高版本)
- [Rust](https://www.rust-lang.org/) (用于 Tauri 构建)
- [npm](https://www.npmjs.com/) 或 [yarn](https://yarnpkg.com/)

### 构建步骤

1. 克隆仓库
   ```bash
   git clone https://github.com/yourusername/MikuModsManager.git
   cd MikuModsManager
   ```

2. 安装依赖
   ```bash
   npm install
   ```

3. 构建开发版本
   ```bash
   npm run tauri dev
   ```

4. 构建生产版本
   ```bash
   npm run tauri build
   ```

构建完成后，可执行文件将位于 `src-tauri/target/release` 目录下。

## 📖 使用方法

### 添加游戏

1. 点击主界面上的「添加游戏」按钮
2. 输入游戏名称并选择游戏安装目录
3. 点击「添加游戏」按钮完成添加

### 安装补丁

1. 在游戏详情页面中，确保补丁已放置在 `game/[游戏名称]/patch/` 目录下
2. 选择要安装的补丁
3. 点击「安装选中补丁」按钮

### 回滚补丁

1. 在游戏详情页面中，选择要回滚到的备份
2. 点击「回滚到选中备份」按钮

## 📁 项目结构

```
MikuModsManager/
├── .gitignore
├── README.md
├── index.html
├── package.json
├── public/
│   ├── MikuMods.png
│   └── ...
├── src/
│   ├── App.vue
│   ├── main.ts
│   ├── components/
│   │   ├── Sidebar.vue
│   │   ├── ContentPanel.vue
│   │   └── panels/
│   │       ├── ModsPanel.vue
│   │       └── GameDetailPanel.vue
│   └── ...
├── src-tauri/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   └── ...
└── ...
```

## 🤝 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📝 许可证

本项目采用 MIT 许可证 - 详情请见 [LICENSE](LICENSE) 文件

## 💬 联系我们

- 项目负责人: [Your Name](https://github.com/yourusername)
- 反馈问题: [Issues](https://github.com/yourusername/MikuModsManager/issues)

## ❤️ 鸣谢

- [Vue.js](https://vuejs.org/)
- [TypeScript](https://www.typescriptlang.org/)
- [Tauri](https://tauri.app/)

---

如果您觉得这个项目有用，请给我们一个 ⭐️ 支持一下！
