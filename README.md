# Cosense Desktop App PoC with Tauri 2.0

A proof-of-concept desktop application built with Tauri 2.0, Vue 3, and Rust that provides an enhanced browsing experience for Cosense (Scrapbox) projects. This application demonstrates Tauri's capabilities for building lightweight, secure, and performant desktop applications.

## ✨ Features

### 🌐 WebView Management
- **Multiple WebView Windows**: Create and manage multiple Scrapbox project windows
- **Navigation Tracking**: Automatic tracking of page navigation and history
- **Recent Windows**: Keep track of recently visited pages for quick access

### ⭐ Favorites System
- **Context Menu Integration**: Right-click on any Scrapbox page to add to favorites
- **WebView Cookie Authentication**: Seamless access to private Scrapbox projects
- **Persistent Storage**: Favorites and recent windows saved using localStorage

### 📚 Scrapbox API Integration
- **Project Page Listing**: Fetch and display pages from any Scrapbox project
- **Authentication Support**: Access private projects using WebView cookies
- **Sorting Options**: Sort pages by update date, creation date, views, or title
- **Real-time Updates**: Refresh page lists with loading states

### 🎨 Modern UI/UX
- **Dark/Light Mode**: Automatic theme switching based on OS preferences
- **Tab-based Interface**: Intuitive tab management similar to modern browsers
- **Responsive Design**: Optimized for desktop usage
- **Loading States**: Visual feedback during API operations

### 🔧 Advanced Features
- **JavaScript Injection**: Custom context menus and page interactions
- **Real-time Notifications**: Success/error feedback for user actions
- **Cross-platform**: Built with Tauri for macOS, Windows, and Linux

## 🏗️ Technical Architecture

### Frontend Stack
- **Vue 3** with Composition API
- **TypeScript** for type safety
- **Vite** for fast development and building
- **CSS Variables** for theme management

### Backend Stack
- **Rust** for secure and performant backend operations
- **Tauri** for desktop application framework
- **reqwest** for HTTP API calls
- **serde** for JSON serialization/deserialization

### Key Components
- **WebView Navigation Tracker**: JavaScript injection for page monitoring
- **Cookie Authentication**: Leveraging WebView cookies for private project access
- **Event-driven Communication**: Rust-JavaScript communication via Tauri commands

## 📊 Performance Metrics

- **Binary Size**: ~11MB (ARM64 macOS)
- **DMG Package**: ~3.9MB (compressed)
- **Memory Usage**: ~30-50MB (vs ~100-200MB for Electron)
- **Startup Time**: Fast startup using OS native WebView

### Comparison with Electron
| Metric | Tauri PoC | Typical Electron App |
|---------|-----------|---------------------|
| Binary Size | 11MB | 100-300MB |
| Memory Usage | 30-50MB | 100-200MB |
| WebView Engine | OS Native | Bundled Chromium |
| Security | Rust + OS WebView | Node.js + Chromium |

## 🚀 Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://rustup.rs/) (latest stable)
- Platform-specific requirements:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools
  - **Linux**: WebKit2GTK development packages

### Installation
```bash
# Clone the repository
git clone https://github.com/kondoumh/sbe-tauri-poc.git
cd sbe-tauri-poc

# Install dependencies
npm install

# Start development server
npm run tauri dev
```

### Building for Production
```bash
# Build the application
npm run tauri build

# The built application will be available in:
# src-tauri/target/release/bundle/
```

## 🔧 Development

### Project Structure
```
├── src/                    # Vue 3 frontend source
│   ├── App.vue            # Main application component
│   └── main.ts            # Vue application entry point
├── src-tauri/             # Rust backend source
│   ├── src/
│   │   └── lib.rs         # Main Rust application logic
│   ├── scripts/
│   │   └── navigation-tracker.js  # WebView JavaScript injection
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node.js dependencies
```

### Key Files
- **`src/App.vue`**: Main Vue component with tab management and UI
- **`src-tauri/src/lib.rs`**: Rust backend with Tauri commands and WebView management
- **`src-tauri/scripts/navigation-tracker.js`**: Injected JavaScript for WebView interaction

### Available Commands
```bash
npm run dev          # Start Vite development server
npm run build        # Build Vue application
npm run tauri dev    # Start Tauri development with hot reload
npm run tauri build  # Build production Tauri application
```

## 🌟 Implemented Features Deep Dive

### WebView Cookie Authentication
The application automatically uses WebView cookies to authenticate with private Scrapbox projects, eliminating the need for manual authentication setup.

### Context Menu Integration
Custom right-click context menus are implemented using JavaScript injection, providing seamless integration with Scrapbox pages.

### Dark Mode Support
Automatic theme switching based on OS preferences using CSS media queries and CSS custom properties.

### Navigation Tracking
Real-time monitoring of WebView navigation events to maintain an accurate history of visited pages.

## 🤝 Contributing

This is a proof-of-concept project demonstrating Tauri's capabilities. Contributions and suggestions are welcome!

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing desktop application framework
- [Vue 3](https://vuejs.org/) - For the reactive frontend framework
- [Scrapbox](https://scrapbox.io/) - For the knowledge management platform
- [Rust](https://www.rust-lang.org/) - For the safe and performant backend language
