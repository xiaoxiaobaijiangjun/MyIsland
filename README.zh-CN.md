<div align="center">
  <img src="resources/icon.png" width="110" alt="MyIsland logo">
  <h1>MyIsland · Windows 灵动岛</h1>
  <p><b>把 macOS 的灵动岛搬到你 Windows 任务栏上方。</b><br>实时歌词、音频可视化、喝水提醒、弹簧动画，一个都不少。</p>

  <p>
    <a href="https://github.com/xiaoxiaobaijiangjun/MyIsland/releases"><img alt="Version" src="https://img.shields.io/badge/版本-1.0.0-00C853?logo=semver&logoColor=white"></a>
    <img alt="Rust" src="https://img.shields.io/badge/Rust-1.96+-CE422B?logo=rust&logoColor=white">
    <img alt="Platform" src="https://img.shields.io/badge/平台-Windows%2010%2F11-0078D4?logo=windows&logoColor=white">
    <img alt="License" src="https://img.shields.io/badge/许可证-GPL--3.0-blue.svg">
    <a href="README.md"><img alt="English" src="https://img.shields.io/badge/README-English-blue.svg"></a>
  </p>

  <p>
    <a href="#-下载"><b>下载</b></a> ·
    <a href="#-功能特性"><b>功能</b></a> ·
    <a href="#-截图"><b>截图</b></a> ·
    <a href="#-插件系统"><b>插件</b></a> ·
    <a href="#-从源码构建"><b>构建</b></a>
  </p>
</div>

> [English](README.md) | **简体中文**

<!-- 🎬 有录屏的话，把 GIF 放到 docs/showcase/hero.gif 然后替换下面的 src —— 动图是涨 star 的头号利器。 -->
<p align="center">
  <img src="docs/social-preview.png" alt="MyIsland — Windows 灵动岛" width="720">
</p>

MyIsland 把 macOS 的**灵动岛**体验带到了 **Windows**。它在你屏幕顶部悬浮一个小小的、带动画的"胶囊"，显示当前播放的音乐——歌名、专辑封面、实时音频可视化，以及**逐行滚动的实时歌词**。它还能提醒你喝水，并支持你自己的插件。

项目用 **Rust** 编写，使用 **Skia** 进行 GPU 加速的 2D 渲染，并通过 Windows 的 [**SMTC**](https://learn.microsoft.com/zh-cn/windows/uwp/audio-video-camera/system-media-transport-controls)（系统媒体传输控件）获取音乐信息——所以它兼容 **Windows 上几乎所有的播放器**，而不是只支持某一个。

---

## ✨ 功能特性

| | 功能 | 说明 |
|---|---|---|
| 🎵 | **音乐 + 实时歌词** | 通过 SMTC 读取任意播放器的当前曲目，歌词随播放逐行平滑滚动，配专辑封面。 |
| 🌊 | **音频可视化** | 基于系统音频的实时频谱条。 |
| 💧 | **喝水提醒** | 可配置提醒间隔（默认 30 分钟）和生效时段，触发时整岛弹窗，让你一眼就看见。 |
| ✨ | **3 种视觉风格** | `默认` · `Mica`（Win11 桌面壁纸取色）· `动态取色`（跟随专辑封面颜色）。 |
| 🪂 | **弹簧动画** | 灵动岛的展开、收起、页面切换全部由弹簧物理驱动，丝滑无卡顿。 |
| 🖱️ | **滚轮切换** | 在灵动岛上滚动鼠标滚轮，即可在「音乐」和「歌词」页面间切换。 |
| 🔌 | **插件系统** | 通过 `myisland-plugin-api` crate 加载外部 `.dll` 插件，自带打包工具（清单 + 签名）。 |
| ⚙️ | **高度可定制** | 全局缩放、停靠位置、字体、语言（EN / 中文）、开机自启等。 |
| 🌍 | **中英双语界面** | 内置英文和简体中文，可在设置中切换。 |

## 📸 截图

<!-- 📸 可选：把真实截图放到 docs/showcase/ 再添加到这里。上面的主图已经展示了 App 的样子，有更多图再补。 -->

## 🚀 下载

在 [**Releases**](https://github.com/xiaoxiaobaijiangjun/MyIsland/releases) 页面下载最新的 **`MyIsland.exe`**，无需安装，双击即可运行。

> 需 **Windows 10 1809+** 或 **Windows 11**。Mica 风格在 Win11 上效果最佳。

## 🎮 使用方法

| 操作 | 效果 |
|---|---|
| **双击**灵动岛 | 展开 / 收起 |
| 在灵动岛上**滚动滚轮** | 在「音乐」↔「歌词」页面间切换 |
| **右键**托盘图标 | 设置 · 显示 / 隐藏 · 退出 |
| **设置 → 通用 → 喝水提醒** | 开启喝水提醒 |

## 🔌 插件系统

MyIsland 在 [`myisland-plugin-api`](crates/myisland-plugin-api) crate 中提供了稳定的插件 API：

```rust
// 基于 myisland-plugin-api 开发插件后，把编译出的 .dll 放到：
//   %APPDATA%/MyIsland/plugins/{你的插件名}/
```

该 crate 还内置了**打包工具**，用于生成插件清单、打包和签名。详见 [`crates/myisland-plugin-api/src/packager`](crates/myisland-plugin-api/src/packager)。

## 🛠️ 从源码构建

**环境要求：** Rust（MSVC 工具链）+ [Visual Studio 2022 Build Tools](https://visualstudio.microsoft.com/zh-hans/downloads/#build-tools-for-visual-studio-2022)（勾选 **C++ 桌面开发**工作负载）。

```bash
rustup default stable-msvc
cargo build --release
# 输出：target/release/MyIsland.exe
```

## ❓ 常见问题

<details>
<summary><b>灵动岛没有显示任何音乐信息。</b></summary>

MyIsland 依赖 Windows 的 SMTC，需要你的播放器把播放状态广播出去。大多数现代播放器（Spotify、Edge、Chrome、装了对应组件的 Foobar2000 等）都支持。如果没显示，确认是不是当前那个播放器持有媒体控制权。
</details>

<details>
<summary><b>Windows Defender / SmartScreen 提示 exe 不安全。</b></summary>

项目目前还没有代码签名证书。对于免费、未签名的下载，这是正常现象——点击 <b>"更多信息 → 仍要运行"</b> 即可。如果你想更稳妥，建议从源码构建（见上文）。
</details>

<details>
<summary><b>设置存在哪里？</b></summary>

配置和插件位于 `%APPDATA%/MyIsland/`。
</details>

## 🗺️ 路线图

- [x] 音乐 + 歌词、视觉风格、喝水提醒、插件 API
- [ ] 番茄钟 / 计时器组件
- [ ] 截图工具集成
- [ ] 更多视觉风格和主题
- [ ] 插件市场 / 自动更新

## 🤝 参与贡献

这是一个年轻的项目，**非常欢迎贡献、Bug 反馈和新想法。**

1. 先开一个 [issue](https://github.com/xiaoxiaobaijiangjun/MyIsland/issues) 讨论你想改的内容
2. Fork → 新建分支 → 用清晰的 commit 信息提交
3. 提交 Pull Request

## 📄 许可证

[GPL-3.0](LICENSE)。MyIsland 是自由开源软件。

---

<div align="center">
  <sub>如果 MyIsland 让你的桌面变得更好看一点，欢迎点个 ⭐ ——这能帮更多人发现它。</sub>
</div>
