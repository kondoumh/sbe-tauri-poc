<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Tab {
  id: string;
  title: string;
  url: string;
  isLoading: boolean;
  favicon?: string;
}

const tabs = ref<Tab[]>([]);
const activeTabId = ref<string | null>(null);
const urlInput = ref("https://example.com");
const errorMessage = ref("");

const activeTab = computed(() => 
  tabs.value.find(tab => tab.id === activeTabId.value)
);

const createNewTab = async (url: string, title?: string) => {
  const tabId = `tab-${Date.now()}`;
  const tab: Tab = {
    id: tabId,
    title: title || new URL(url).hostname,
    url,
    isLoading: true,
    favicon: undefined
  };

  tabs.value.push(tab);
  activeTabId.value = tabId;

  try {
    // Create the webview content in this tab
    await invoke('create_tab_content', { 
      tabId,
      url
    });
    
    // Update tab title based on page content (simplified)
    if (url.includes('scrapbox.io')) {
      tab.title = url.includes('/') ? 
        url.split('/').pop() || 'Scrapbox' : 'Scrapbox';
    }
    
    // Set a timeout to stop loading animation
    setTimeout(() => {
      tab.isLoading = false;
    }, 2000);
    
  } catch (error) {
    console.error('Failed to create tab:', error);
    errorMessage.value = `Failed to create tab: ${error}`;
    tab.isLoading = false;
    // Remove failed tab
    tabs.value = tabs.value.filter(t => t.id !== tabId);
    activeTabId.value = tabs.value.length > 0 ? tabs.value[tabs.value.length - 1].id : null;
  }
};

const handleIframeError = (tabId: string) => {
  const tab = tabs.value.find(t => t.id === tabId);
  if (tab) {
    tab.isLoading = false;
    errorMessage.value = `このサイトはiframeでの表示が制限されています。ブラウザで開くボタンを使用してください。`;
  }
};

const openTabInNewWindow = async (tabId: string) => {
  const tab = tabs.value.find(t => t.id === tabId);
  if (tab) {
    try {
      await invoke('create_webview_window', { 
        url: tab.url,
        label: `window-${tabId}`
      });
    } catch (error) {
      console.error('Failed to open in new window:', error);
      errorMessage.value = `Failed to open in new window: ${error}`;
    }
  }
};

// Scrapbox用の専用機能
const openScrapboxTab = async () => {
  const scrapboxUrl = "https://scrapbox.io";
  try {
    await invoke('create_webview_window', { 
      url: scrapboxUrl,
      label: `scrapbox-${Date.now()}`
    });
    errorMessage.value = "ScrapboxページをWebViewウィンドウで開きました";
  } catch (error) {
    console.error('Failed to open Scrapbox:', error);
    errorMessage.value = `Scrapboxの起動に失敗しました: ${error}`;
  }
};

const closeTab = (tabId: string) => {
  const tabIndex = tabs.value.findIndex(tab => tab.id === tabId);
  if (tabIndex === -1) return;

  // Remove tab
  tabs.value.splice(tabIndex, 1);

  // Update active tab
  if (activeTabId.value === tabId) {
    if (tabs.value.length > 0) {
      // Switch to previous tab or first available
      const newActiveIndex = Math.max(0, tabIndex - 1);
      activeTabId.value = tabs.value[newActiveIndex]?.id || null;
    } else {
      activeTabId.value = null;
    }
  }

  // Notify backend to clean up
  invoke('close_tab', { tabId }).catch(console.error);
};

const switchTab = (tabId: string) => {
  activeTabId.value = tabId;
  // Notify backend about tab switch
  invoke('switch_to_tab', { tabId }).catch(console.error);
};

const navigateActiveTab = async () => {
  if (!activeTab.value) {
    // Create new tab if none exists
    await createNewTab(urlInput.value);
    return;
  }

  try {
    activeTab.value.isLoading = true;
    errorMessage.value = "";
    activeTab.value.url = urlInput.value;
    
    await invoke('navigate_tab', { 
      tabId: activeTab.value.id,
      url: urlInput.value
    });
    
    // Update tab title
    activeTab.value.title = new URL(urlInput.value).hostname;
    
  } catch (error) {
    console.error('Navigation error:', error);
    errorMessage.value = `Navigation failed: ${error}`;
  } finally {
    if (activeTab.value) {
      activeTab.value.isLoading = false;
    }
  }
};

const addNewTab = async () => {
  await createNewTab("https://example.com");
  urlInput.value = "https://example.com";
};

const goHome = async () => {
  urlInput.value = "https://example.com";
  await navigateActiveTab();
};

const openInBrowser = async () => {
  try {
    const url = activeTab.value?.url || urlInput.value;
    await invoke('open_url', { url });
  } catch (error) {
    console.error('Failed to open in browser:', error);
    errorMessage.value = `Failed to open in browser: ${error}`;
  }
};

const refresh = async () => {
  if (activeTab.value) {
    urlInput.value = activeTab.value.url;
    await navigateActiveTab();
  }
};

onMounted(async () => {
  // Create initial tab with a site that allows iframe embedding
  await createNewTab("https://example.com", "Example");
});
</script>

<template>
  <div class="app-container">
    <header class="app-header">
      <div class="nav-controls">
        <button @click="refresh" class="nav-btn" title="更新" :disabled="!activeTab || activeTab.isLoading">⟳</button>
        <button @click="goHome" class="nav-btn" title="ホーム" :disabled="!activeTab || activeTab.isLoading">🏠</button>
        <button @click="openInBrowser" class="nav-btn" title="ブラウザで開く">🔗</button>
        <button @click="addNewTab" class="nav-btn" title="新しいタブ">+</button>
        <button @click="openScrapboxTab" class="nav-btn scrapbox-btn" title="Scrapboxを開く">📝</button>
      </div>
      <div class="url-bar">
        <input 
          v-model="urlInput" 
          @keyup.enter="navigateActiveTab"
          class="url-input"
          placeholder="URLを入力してください"
          :disabled="!activeTab || activeTab.isLoading"
        />
        <button @click="navigateActiveTab" class="go-btn" :disabled="!activeTab || activeTab.isLoading">
          {{ activeTab?.isLoading ? '読み込み中...' : '移動' }}
        </button>
      </div>
    </header>

    <!-- Tab Bar -->
    <div class="tab-bar" v-if="tabs.length > 0">
      <div 
        v-for="tab in tabs" 
        :key="tab.id"
        :class="['tab', { active: tab.id === activeTabId, loading: tab.isLoading }]"
        @click="switchTab(tab.id)"
      >
        <span class="tab-favicon" v-if="tab.favicon">{{ tab.favicon }}</span>
        <span class="tab-title">{{ tab.title }}</span>
        <button 
          @click.stop="closeTab(tab.id)" 
          class="tab-close"
          title="タブを閉じる"
        >×</button>
      </div>
    </div>
    
    <main class="content-container">
      <div v-if="activeTab" class="tab-content">
        <div class="tab-info">
          <div class="current-url">
            現在のURL: <code>{{ activeTab.url }}</code>
          </div>
          
          <div v-if="activeTab.isLoading" class="loading-message">
            読み込み中...
          </div>
        </div>
        
        <!-- WebView content area -->
        <div class="webview-container" :key="activeTab.id">
          <iframe 
            :src="activeTab.url"
            class="webview-frame"
            frameborder="0"
            allowfullscreen
            @error="handleIframeError(activeTab.id)"
            @load="activeTab.isLoading = false"
          ></iframe>
          
          <!-- Iframe alternative overlay -->
          <div v-if="errorMessage && errorMessage.includes('iframe')" class="iframe-blocked-overlay">
            <div class="blocked-content">
              <h3>🚫 サイトが表示できません</h3>
              <p>このサイトはセキュリティ上の理由でiframe内での表示が制限されています。</p>
              <div class="alternative-actions">
                <button @click="openTabInNewWindow(activeTab.id)" class="alt-btn">
                  新しいウィンドウで開く
                </button>
                <button @click="openInBrowser" class="alt-btn">
                  ブラウザで開く
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div v-else class="no-tabs">
        <h2>SBE - Scrapbox Desktop Client</h2>
        <p>新しいタブを作成してScrapboxを使い始めましょう。</p>
        <button @click="addNewTab" class="create-tab-btn">最初のタブを作成</button>
      </div>
      
      <div v-if="errorMessage" class="error-message">
        {{ errorMessage }}
      </div>
      
      <div class="instructions">
        <h3>使い方:</h3>
        <ul>
          <li>📝 ボタンでScrapboxを専用WebViewウィンドウで開く（推奨）</li>
          <li>+ ボタンで新しいタブを作成（テスト用）</li>
          <li>タブをクリックして切り替え</li>
          <li>× ボタンでタブを閉じる</li>
          <li>URLを入力して「移動」でナビゲーション</li>
          <li>🏠 ボタンでホームページに戻る</li>
          <li>🔗 ボタンでデフォルトブラウザで開く</li>
        </ul>
        <p><strong>注意:</strong> ScrapboxはiFrame表示が制限されているため、📝ボタンで専用ウィンドウを使用してください。</p>
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

.scrapbox-btn {
  background-color: #28a745;
  color: white;
  border-color: #28a745;
}

.scrapbox-btn:hover {
  background-color: #218838;
  border-color: #1e7e34;
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

/* Tab Bar Styles */
.tab-bar {
  display: flex;
  background-color: #e9e9e9;
  border-bottom: 1px solid #ccc;
  overflow-x: auto;
  flex-shrink: 0;
}

.tab {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border-right: 1px solid #ccc;
  background-color: #f5f5f5;
  cursor: pointer;
  transition: background-color 0.2s;
  min-width: 180px;
  max-width: 200px;
  user-select: none;
  position: relative;
}

.tab:hover {
  background-color: #e0e0e0;
}

.tab.active {
  background-color: #fff;
  border-bottom: 2px solid #007acc;
}

.tab.loading {
  opacity: 0.7;
}

.tab.loading::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, #007acc, transparent);
  animation: loading 1.5s infinite;
}

@keyframes loading {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(200%); }
}

.tab-favicon {
  margin-right: 6px;
  font-size: 14px;
}

.tab-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
  color: #333;
}

.tab.active .tab-title {
  font-weight: 500;
}

.tab-close {
  background: none;
  border: none;
  color: #666;
  font-size: 16px;
  cursor: pointer;
  padding: 0;
  margin-left: 8px;
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
  transition: all 0.2s;
}

.tab-close:hover {
  background-color: #ff4444;
  color: white;
}

.content-container {
  flex: 1;
  overflow: hidden;
  background-color: #fafafa;
  display: flex;
  flex-direction: column;
}

/* Tab Content Styles */
.tab-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.tab-info {
  padding: 12px 20px;
  background-color: #f9f9f9;
  border-bottom: 1px solid #ddd;
  flex-shrink: 0;
}

.webview-container {
  flex: 1;
  margin: 0;
  padding: 0;
  background-color: #fff;
  position: relative;
  overflow: hidden;
}

.webview-frame {
  width: 100%;
  height: 100%;
  border: none;
  background-color: #fff;
}

.iframe-blocked-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(255, 255, 255, 0.95);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

.blocked-content {
  text-align: center;
  max-width: 400px;
  padding: 32px;
  background-color: white;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.blocked-content h3 {
  color: #d32f2f;
  margin-bottom: 16px;
  font-size: 20px;
}

.blocked-content p {
  color: #666;
  margin-bottom: 24px;
  line-height: 1.5;
}

.alternative-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.alt-btn {
  background: #007acc;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 10px 16px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.2s;
}

.alt-btn:hover {
  background-color: #005a9e;
}

.no-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 40px;
}

.no-tabs h2 {
  color: #333;
  margin-bottom: 20px;
  font-size: 28px;
}

.create-tab-btn {
  background: #007acc;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 12px 24px;
  cursor: pointer;
  font-size: 16px;
  margin-top: 20px;
  transition: background-color 0.2s;
}

.create-tab-btn:hover {
  background-color: #005a9e;
}

.current-url {
  margin: 0;
  padding: 8px 0;
  font-size: 13px;
  color: #666;
}

.current-url code {
  color: #007acc;
  font-weight: 500;
  font-size: 12px;
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
  padding: 8px 0;
  color: #0066cc;
  font-size: 13px;
}

.instructions {
  margin: 20px;
  padding: 20px;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  flex-shrink: 0;
}

.instructions h3 {
  color: #333;
  margin-bottom: 12px;
  font-size: 16px;
}

.instructions ul {
  list-style-type: disc;
  padding-left: 20px;
}

.instructions li {
  margin-bottom: 6px;
  line-height: 1.4;
  font-size: 14px;
}

.instructions p {
  margin-top: 16px;
  padding: 12px;
  background-color: #fff3cd;
  border: 1px solid #ffeaa7;
  border-radius: 6px;
  color: #856404;
  font-size: 13px;
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