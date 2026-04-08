# 🐱 Neko Downloader

A modern, fast, and efficient download manager built with **Tauri** (Rust + JavaScript).

![Tauri](https://img.shields.io/badge/Tauri-1.5-blue)
![Rust](https://img.shields.io/badge/Rust-1.75-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## ✨ Features

- 🚀 **Fast & Lightweight** - Built with Tauri, much smaller than Electron
- 📊 **Real-time Progress** - Live download speed and progress tracking
- 🎨 **Modern UI** - Dark theme with smooth animations
- 💾 **Smart Downloads** - Resume support and chunked downloading
- 🔒 **Secure** - Rust backend ensures memory safety
- 🪟 **Custom Window** - Frameless window with custom controls
- 📱 **Responsive** - Works on all screen sizes

## 🛠️ Tech Stack

### Backend (Rust)
- **Tauri** - Framework for building desktop apps
- **Tokio** - Async runtime
- **Reqwest** - HTTP client with streaming support
- **Serde** - Serialization/deserialization

### Frontend (JavaScript)
- **Bootstrap 5** - UI framework
- **Font Awesome** - Icons
- **Tauri API** - Communication with Rust backend

## 📦 Installation

### Prerequisites

1. **Rust** (1.75 or higher)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Node.js** (18 or higher)
```bash
# Download from https://nodejs.org/
```

3. **System Dependencies**

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Build & Run

1. **Clone or navigate to project:**
```bash
cd neko-downloader
```

2. **Install Node dependencies:**
```bash
npm install
```

3. **Run in development mode:**
```bash
npm run tauri dev
```

4. **Build for production:**
```bash
npm run tauri build
```

The compiled app will be in `src-tauri/target/release/bundle/`

## 🚀 Usage

1. **Start the app**
2. **Enter a download URL** in the input field
3. **Click "Download" button**
4. **Choose save location** in the file dialog
5. **Watch your download progress** in real-time!

### Keyboard Shortcuts
- `Ctrl/Cmd + Q` - Quit app
- `Ctrl/Cmd + M` - Minimize window
- `Ctrl/Cmd + W` - Close window

## 📁 Project Structure

```
neko-downloader/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   └── main.rs        # Main Rust code with download logic
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── build.rs           # Build script
├── ui/                    # Frontend
│   └── index.html         # Main HTML/CSS/JS
├── package.json           # Node.js dependencies
└── README.md             # This file
```

## 🔧 Configuration

Edit `src-tauri/tauri.conf.json` to customize:
- Window size and behavior
- App permissions
- Build settings
- Icon and metadata

## 🎯 Roadmap

- [ ] Pause/Resume downloads
- [ ] Multi-threaded downloads (chunking)
- [ ] Browser extension integration
- [ ] Download scheduling
- [ ] Torrent support
- [ ] Batch downloads from file
- [ ] Download history
- [ ] Bandwidth limiter
- [ ] Auto-retry on failure
- [ ] Proxy support

## 🐛 Known Issues

- Large files (>2GB) may need chunking implementation
- Some websites may block automated downloads
- SSL certificate validation on some sites

## 🤝 Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📄 License

MIT License - see LICENSE file for details

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Amazing framework
- [Bootstrap](https://getbootstrap.com/) - UI components
- [Font Awesome](https://fontawesome.com/) - Icons

## 📞 Support

If you encounter any issues or have questions:
- Open an issue on GitHub
- Check existing issues for solutions

---

**Made with ❤️ using Tauri**