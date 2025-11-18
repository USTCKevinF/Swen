<div align="center">
  <img src="./public/logo/logo.png" alt="Swen Logo" width="150"/>
  
  # Swen
  
  ### A Cross-Platform Desktop Assistant for Instant and Personalized LLM Interaction
  
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-lightgrey.svg)]()
  [![Tauri](https://img.shields.io/badge/Tauri-2.0-orange.svg)](https://tauri.app/)
  
  English | [简体中文](README.md)
  
</div>

---

## ✨ Introduction

**Swen** is a cross-platform desktop intelligent assistant designed to seamlessly integrate the powerful capabilities of Large Language Models (LLMs) into your daily workflow. No more frequent app switching, no more manual copy-pasting—simply select any text and press a shortcut key to receive instant, personalized AI assistance.

### 💡 Core Philosophy

Traditional LLM usage requires users to constantly switch applications, copy text, and manually craft prompts. These repetitive operations not only disrupt cognitive flow but also significantly reduce learning and work efficiency. Swen embeds LLM capabilities directly into your operating system environment, making AI assistance **continuous, intelligent, and seamless**.

---

## 🎯 Key Features

### 1️⃣ Seamless Interaction
- **One-Key Invocation**: Select text + shortcut key = instant AI response
- **No Context Switching**: Use directly within your current application without interrupting workflow
- **Cross-Platform**: Unified experience on Windows and macOS

### 2️⃣ Intelligent Input Capture
- **Text Selection Mode**: Directly select text in any editable area
- **OCR Recognition Mode**: Automatically switches to OCR for PDFs, images, and scanned documents
- **Smart Mode Switching**: System automatically determines the appropriate mode

### 3️⃣ Automatic Intent Recognition
Automatically identifies user intent based on context, no prompt engineering required:
- 📝 **Summarization**: Auto-generates summaries for long texts
- 🌐 **Translation**: Instant translation of foreign terms
- 💡 **Explanation**: In-depth explanation of technical concepts

### 4️⃣ Personalized Knowledge Engine
Continuously learns your preferences to provide tailored responses:
- **Domain Interest**: Adjusts explanations based on your professional background
- **Knowledge Level**: Adapts explanation depth automatically
- **Language Preference**: Maintains consistent terminology and output language
- **Response Style**: Concise, detailed, or example-driven—adapts to your style

### 5️⃣ Rich Text Rendering
- Markdown format support
- Code syntax highlighting
- Mathematical formula rendering
- Mermaid diagram generation

---

## 🏗️ System Architecture

<div align="center">
  <img src="./public/main/system_workflow.png" alt="System Workflow" width="100%"/>
  <p><i>Swen System Architecture</i></p>
</div>

### Core Modules

1. **Input Acquisition Module**
   - System-level text selection capture
   - OCR text recognition

2. **Intent Recognition & Personalization Engine**
   - Automatic intent analysis (Translation/Explanation/Summarization)
   - User profile maintenance and updates
   - Personalized response generation

3. **Generate & Render**
   - Markdown rich text rendering
   - Code block syntax highlighting
   - Mathematical formulas and diagram support

---

## 🚀 Usage Guide

<div align="center">
  <img src="./public/main/user_workflow.png" alt="User Workflow" width="100%"/>
  <p><i>User Workflow Diagram</i></p>
</div>

### Quick Start

1. **Configure API**
   - Set up your LLM API on first launch (supports OpenAI, Claude, etc.)
   - Configure your shortcut key (default: `Ctrl/Cmd + Shift + A`)

2. **Text Selection Mode**
   ```
   Select any text → Press shortcut → Get AI response
   ```
   Use cases: Web pages, documents, code editors, and any selectable text

3. **OCR Recognition Mode**
   ```
   Press shortcut → Draw selection area on screen → Auto-recognize text and respond
   ```
   Use cases: PDFs, images, scanned documents, and non-selectable content

4. **Continuous Conversation**
   - Continue asking questions in the popup window
   - Multi-turn dialogue with context preservation

### Usage Examples

#### Scenario 1: Technical Documentation Learning
```
Select "KL divergence" → Press shortcut
```
Swen automatically recognizes **Explanation** intent and provides targeted explanation based on your background (e.g., machine learning), including mathematical definitions and code implementation.

#### Scenario 2: Foreign Language Reading
```
Select "Taylor polynomials" → Press shortcut
```
If your language is set to Chinese, Swen automatically recognizes **Translation** intent and provides translation with a brief explanation.

#### Scenario 3: Long Text Summarization
```
Select a long paper abstract → Press shortcut
```
Swen automatically recognizes **Summarization** intent, extracts key points, and generates a concise summary.

---

## 📦 Installation

### Install from Release (Recommended)

1. Visit the [Releases](https://github.com/yourusername/swen/releases) page
2. Download the installer for your platform:
   - **Windows**: `Swen-setup.exe`
   - **macOS**: `Swen.dmg`
3. Run the installer and follow the prompts

### Build from Source

```bash
# Clone repository
git clone https://github.com/yourusername/swen.git
cd swen

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build production version
npm run tauri build
```

---

## 🛠️ Tech Stack

- **Framework**: [Tauri 2.0](https://tauri.app/) - Lightweight cross-platform desktop framework
- **Backend**: [Rust](https://www.rust-lang.org/) - High-performance system-level operations
- **Frontend**: [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/)
- **Database**: SQLite - Local storage for configuration and history
- **OCR Engine**: Embedded OCR model
- **UI Components**: Markdown, code highlighting, mathematical formulas, Mermaid diagrams

---

## 🔒 Privacy & Security

- ✅ All user profile data **encrypted and stored locally**
- ✅ No personal data uploaded to third-party servers
- ✅ API keys securely stored locally
- ✅ History can be cleared at any time

---

## 📖 Configuration

### API Configuration
Supports mainstream LLM providers:
- OpenAI (GPT-3.5, GPT-4)
- Anthropic (Claude)
- Other OpenAI API-compatible services

### Shortcut Key Settings
Customize shortcut key combinations in settings.

### Personalization Options
- Language preference
- Response detail level
- Default intent recognition behavior

---

## 🗺️ Roadmap

- [x] Basic text selection capture
- [x] OCR recognition mode
- [x] Automatic intent recognition
- [x] Personalization engine
- [x] Cross-platform support (Windows, macOS)
- [ ] Linux support
- [ ] Multi-language interface
- [ ] Plugin system
- [ ] Local model support

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit code, report issues, or suggest improvements.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👨‍💻 Author

**Wenjun Feng**
- Institution: University of Science and Technology of China
- Email: fengwenjun@mail.ustc.edu.cn

---

## 🙏 Acknowledgments

Thanks to all developers who contribute to the open-source community, and the following projects:
- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Rust](https://www.rust-lang.org/)

---

<div align="center">
  
  **If you find Swen helpful, please consider giving it a Star ⭐!**
  
  Made with ❤️ by Wenjun Feng
  
</div>

