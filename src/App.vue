<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const currentUrl = ref("https://scrapbox.io");
const urlInput = ref("https://scrapbox.io");
const isLoading = ref(false);
const errorMessage = ref("");

const navigateToUrl = async () => {
  try {
    isLoading.value = true;
    errorMessage.value = "";
    currentUrl.value = urlInput.value;
    
    // Use Tauri command to create new webview window
    await invoke('create_webview_window', { 
      url: currentUrl.value,
      label: `scrapbox-${Date.now()}` // unique label for each window
    });
    
  } catch (error) {
    console.error('Navigation error:', error);
    errorMessage.value = `Navigation failed: ${error}`;
  } finally {
    isLoading.value = false;
  }
};

const goHome = () => {
  urlInput.value = "https://scrapbox.io";
  navigateToUrl();
};

const openInBrowser = async () => {
  try {
    await invoke('open_url', { url: currentUrl.value });
  } catch (error) {
    console.error('Failed to open in browser:', error);
    errorMessage.value = `Failed to open in browser: ${error}`;
  }
};

const refresh = () => {
  navigateToUrl();
};

onMounted(() => {
  // Don't auto-navigate on mount, let user choose
});
</script>

<template>
  <div class="app-container">
    <header class="app-header">
      <div class="nav-controls">
        <button @click="refresh" class="nav-btn" title="更新" :disabled="isLoading">⟳</button>
        <button @click="goHome" class="nav-btn" title="ホーム" :disabled="isLoading">🏠</button>
        <button @click="openInBrowser" class="nav-btn" title="ブラウザで開く">🔗</button>
      </div>
      <div class="url-bar">
        <input 
          v-model="urlInput" 
          @keyup.enter="navigateToUrl"
          class="url-input"
          placeholder="URLを入力してください"
          :disabled="isLoading"
        />
        <button @click="navigateToUrl" class="go-btn" :disabled="isLoading">
          {{ isLoading ? '読み込み中...' : '移動' }}
        </button>
      </div>
    </header>
    
    <main class="content-container">
      <div class="status-section">
        <h2>SBE - Scrapbox Desktop Client</h2>
        <div class="current-url">
          現在のURL: <code>{{ currentUrl }}</code>
        </div>
        
        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>
        
        <div v-if="isLoading" class="loading-message">
          新しいウィンドウを開いています...
        </div>
        
        <div class="instructions">
          <h3>使い方:</h3>
          <ul>
            <li>URLを入力して「移動」ボタンを押すと、新しいWebViewウィンドウでサイトが開きます</li>
            <li>🏠 ボタンでScrapboxホームに戻ります</li>
            <li>⟳ ボタンで現在のURLを再読み込みします</li>
            <li>🔗 ボタンでデフォルトブラウザでURLを開きます</li>
          </ul>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body, html {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  height: 100%;
  overflow: hidden;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}

.app-header {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  background-color: #f5f5f5;
  border-bottom: 1px solid #ddd;
  gap: 12px;
  flex-shrink: 0;
}

.nav-controls {
  display: flex;
  gap: 4px;
}

.nav-btn {
  background: #fff;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 6px 10px;
  cursor: pointer;
  font-size: 14px;
  min-width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.nav-btn:hover {
  background-color: #e9e9e9;
  border-color: #999;
}

.nav-btn:active {
  background-color: #ddd;
}

.url-bar {
  flex: 1;
  display: flex;
  gap: 8px;
  align-items: center;
}

.url-input {
  flex: 1;
  padding: 6px 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
  height: 32px;
  outline: none;
}

.url-input:focus {
  border-color: #007acc;
  box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.2);
}

.go-btn {
  background: #007acc;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 6px 16px;
  cursor: pointer;
  font-size: 14px;
  height: 32px;
  transition: background-color 0.2s;
}

.go-btn:hover {
  background-color: #005a9e;
}

.content-container {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  background-color: #fafafa;
}

.status-section {
  max-width: 800px;
  margin: 0 auto;
}

.status-section h2 {
  color: #333;
  margin-bottom: 20px;
  font-size: 28px;
}

.current-url {
  margin: 16px 0;
  padding: 12px;
  background-color: #f0f0f0;
  border-radius: 6px;
  font-size: 14px;
}

.current-url code {
  color: #007acc;
  font-weight: 500;
}

.error-message {
  margin: 16px 0;
  padding: 12px;
  background-color: #ffeaea;
  border: 1px solid #ffcccc;
  border-radius: 6px;
  color: #d8000c;
}

.loading-message {
  margin: 16px 0;
  padding: 12px;
  background-color: #e6f3ff;
  border: 1px solid #b3d9ff;
  border-radius: 6px;
  color: #0066cc;
}

.instructions {
  margin-top: 24px;
  padding: 20px;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.instructions h3 {
  color: #333;
  margin-bottom: 12px;
}

.instructions ul {
  list-style-type: disc;
  padding-left: 20px;
}

.instructions li {
  margin-bottom: 8px;
  line-height: 1.4;
}

.nav-btn:disabled,
.go-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.url-input:disabled {
  background-color: #f5f5f5;
  cursor: not-allowed;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>