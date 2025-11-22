<template>
  <div class="app">
    <!-- Tab bar -->
    <div class="tab-bar">
      <div v-for="tab in tabs" :key="tab.id" 
           class="tab" 
           :class="{ active: tab.id === activeTabId }"
           @click="switchTab(tab.id)">
        <span class="tab-icon">{{ tab.icon }}</span>
        <span class="tab-title">{{ tab.title }}</span>
        <button v-if="tab.closable" 
                class="close-btn" 
                @click.stop="closeTab(tab.id)">×</button>
      </div>
      <button class="add-tab-btn" @click="showNewTabDialog = true">+</button>
    </div>

    <!-- Error message -->
    <div v-if="errorMessage" class="error-message">
      {{ errorMessage }}
    </div>

    <!-- Tab content -->
    <div class="tab-content">
      <!-- Manager Tab -->
      <div v-if="activeTab?.component === 'manager'" class="manager-view">
        <div class="scrapbox-home-section">
          <button @click="openScrapboxHome" class="scrapbox-home-btn">
            <span class="btn-icon">📦</span>
            <span class="btn-text">Scrapbox ホームを開く</span>
          </button>
        </div>
        
        <div class="section">
          <h2>📋 最近のウィンドウ</h2>
          <div v-if="recentWindows.length === 0" class="empty-state">
            まだウィンドウが開かれていません
          </div>
          <div v-for="window in recentWindows" :key="window.id" class="window-item">
            <div class="window-info">
              <div class="window-title">{{ window.title }}</div>
              <div class="window-url">{{ window.url }}</div>
              <div class="window-time">{{ formatTime(window.lastAccessed) }}</div>
            </div>
            <div class="window-actions">
              <button @click="reopenWindow(window)" class="action-btn">再開</button>
              <button @click="removeFromRecent(window.id)" class="remove-btn">削除</button>
            </div>
          </div>
        </div>

        <div class="section">
          <h2>⭐ お気に入り</h2>
          <div class="favorite-instruction">
            📝 WebViewでScrapboxページを右クリックして「お気に入りに追加」で追加できます
          </div>
          <div v-if="favorites.length === 0" class="empty-state">
            お気に入りが登録されていません
          </div>
          <div v-for="favorite in favorites" :key="favorite.id" class="favorite-item">
            <div class="favorite-info">
              <div class="favorite-title">{{ favorite.title }}</div>
              <div class="favorite-url">{{ favorite.url }}</div>
            </div>
            <div class="favorite-actions">
              <button @click="openFavorite(favorite)" class="action-btn">開く</button>
              <button @click="removeFavorite(favorite.id)" class="remove-btn">削除</button>
            </div>
          </div>
        </div>

      </div>

      <!-- Scrapbox Pages Tab -->
      <div v-if="activeTab?.component === 'scrapbox-pages'" class="scrapbox-pages-view">
        <div class="scrapbox-header">
          <h2>📝 Scrapboxページ一覧</h2>
          <div class="scrapbox-controls">
            <input v-model="scrapboxProject" 
                   @keyup.enter="changeScrapboxProject"
                   class="project-input"
                   placeholder="プロジェクト名" />
            <select v-model="scrapboxSort" @change="fetchScrapboxPages" class="sort-select">
              <option value="updated">更新日時</option>
              <option value="created">作成日時</option>
              <option value="views">閲覧数</option>
              <option value="title">タイトル</option>
            </select>
            <button @click="fetchScrapboxPages" class="refresh-btn" :disabled="scrapboxLoading">
              {{ scrapboxLoading ? '🔄 読み込み中...' : '🔄 更新' }}
            </button>
          </div>
        </div>

        <div v-if="scrapboxError" class="error-message">
          {{ scrapboxError }}
        </div>

        <div class="scrapbox-page-list" v-if="!scrapboxLoading">
          <div class="page-header">
            <div class="page-title-header">タイトル</div>
            <div class="page-views-header">閲覧数</div>
            <div class="page-links-header">被リンク数</div>
            <div class="page-updated-header">更新日時</div>
            <div class="page-user-header">更新者</div>
          </div>
          
          <div v-for="page in scrapboxPages" :key="page.id" 
               class="scrapbox-page-item"
               @click="openScrapboxPage(page)">
            <div class="page-title">{{ page.title }}</div>
            <div class="page-views">{{ page.views.toLocaleString() }}</div>
            <div class="page-links">{{ page.linked }}</div>
            <div class="page-time">{{ formatDate(page.updated * 1000) }}</div>
            <div class="page-user">
              <span class="user-name">{{ page.last_update_user?.id || page.user.id || '-' }}</span>
            </div>
          </div>
        </div>

        <div v-if="scrapboxLoading" class="loading-state">
          読み込み中...
        </div>

        <div v-if="!scrapboxLoading && scrapboxPages.length === 0" class="empty-state">
          ページが見つかりませんでした
        </div>
      </div>

      <!-- WebView Tab -->
      <div v-if="activeTab?.component === 'webview'" class="webview-container">
        <div class="webview-header">
          <input v-model="urlInput" 
                 @keyup.enter="navigateToUrl" 
                 class="url-input"
                 :placeholder="activeTab?.url || 'URLを入力してください'" />
          <button @click="navigateToUrl" class="navigate-btn">移動</button>
        </div>
        
        <iframe v-if="activeTab?.url" 
                :src="activeTab.url" 
                class="webview-iframe"
                @error="handleIframeError">
        </iframe>
        
        <div v-if="activeTab?.isLoading" class="loading-indicator">
          読み込み中...
        </div>
      </div>
    </div>

    <!-- New Tab Dialog -->
    <div v-if="showNewTabDialog" class="modal-overlay" @click="showNewTabDialog = false">
      <div class="modal" @click.stop>
        <h3>新しいタブ</h3>
        <input v-model="newTabUrl" 
               placeholder="URLを入力してください" 
               @keyup.enter="createNewTabFromDialog"
               class="url-input" />
        <div class="modal-actions">
          <button @click="createNewTabFromDialog" class="action-btn">作成</button>
          <button @click="showNewTabDialog = false" class="cancel-btn">キャンセル</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface Tab {
  id: string;
  title: string;
  component: 'manager' | 'webview' | 'scrapbox-pages';
  icon: string;
  url?: string;
  isLoading?: boolean;
  favicon?: string;
  closable?: boolean;
}

interface RecentWindow {
  id: string;
  title: string;
  url: string;
  lastAccessed: Date;
}

interface Favorite {
  id: string;
  title: string;
  url: string;
}

interface ScrapboxPage {
  id: string;
  title: string;
  views: number;
  linked: number;
  updated: number;
  user: {
    id: string;
  };
  last_update_user?: {
    id: string;
  };
}

// Tab management
const tabs = ref<Tab[]>([
  {
    id: 'manager',
    title: '管理',
    component: 'manager',
    icon: '📋',
    closable: false
  },
  {
    id: 'scrapbox-pages',
    title: 'Scrapboxページ一覧',
    component: 'scrapbox-pages',
    icon: '📝',
    closable: false
  }
]);

const activeTabId = ref<string>('manager');
const errorMessage = ref("");

// UI state
const showNewTabDialog = ref(false);
const newTabUrl = ref("");
const urlInput = ref("");

// Manager data
const recentWindows = ref<RecentWindow[]>([]);
const favorites = ref<Favorite[]>([]);

// Scrapbox pages state
const scrapboxProject = ref('help-jp');
const scrapboxPages = ref<ScrapboxPage[]>([]);
const scrapboxLoading = ref(false);
const scrapboxError = ref('');
const scrapboxSkip = ref(0);
const scrapboxLimit = ref(100);
const scrapboxSort = ref('updated');

let tabCounter = 0;

const activeTab = computed(() => 
  tabs.value.find(tab => tab.id === activeTabId.value)
);

// Tab functions
// Tab functions
const switchTab = (tabId: string) => {
  activeTabId.value = tabId;
  
  // Update URL input for webview tabs
  const tab = tabs.value.find(t => t.id === tabId);
  if (tab?.component === 'webview' && tab.url) {
    urlInput.value = tab.url;
  } else {
    urlInput.value = "";
  }
};

const createWebViewTab = async (url: string, title?: string) => {
  try {
    tabCounter++;
    const tabId = `webview-${tabCounter}`;
    const tabTitle = title || new URL(url).hostname;
    
    const newTab: Tab = {
      id: tabId,
      title: tabTitle,
      component: 'webview',
      icon: '🌐',
      url,
      isLoading: true,
      closable: true
    };
    
    tabs.value.push(newTab);
    activeTabId.value = tabId;
    urlInput.value = url;
    
    // Create the webview content
    await invoke('create_tab_content', { 
      tabId,
      url
    });
    
    // Set a timeout to stop loading animation
    setTimeout(() => {
      newTab.isLoading = false;
    }, 2000);
    
    console.log(`Created WebView tab: ${tabTitle}`);
    errorMessage.value = "";
  } catch (error) {
    console.error('Failed to create WebView tab:', error);
    errorMessage.value = `タブの作成に失敗しました: ${error}`;
  }
};

const createNewTabFromDialog = () => {
  if (newTabUrl.value.trim()) {
    createWebViewTab(newTabUrl.value.trim());
    showNewTabDialog.value = false;
    newTabUrl.value = "";
  }
};

const navigateToUrl = () => {
  if (activeTab.value?.component === 'webview' && urlInput.value.trim()) {
    // Update current tab URL
    const tab = activeTab.value;
    tab.url = urlInput.value.trim();
    tab.isLoading = true;
    
    // Navigate in current tab
    invoke('create_tab_content', { 
      tabId: tab.id,
      url: tab.url
    }).then(() => {
      setTimeout(() => {
        if (tab) tab.isLoading = false;
      }, 2000);
    }).catch((error) => {
      console.error('Navigation failed:', error);
      errorMessage.value = `ナビゲーションに失敗しました: ${error}`;
      if (tab) tab.isLoading = false;
    });
  } else if (urlInput.value.trim()) {
    // Create new tab
    createWebViewTab(urlInput.value.trim());
  }
};

const closeTab = (tabId: string) => {
  const tabIndex = tabs.value.findIndex(tab => tab.id === tabId);
  if (tabIndex > -1) {
    const tab = tabs.value[tabIndex];
    if (!tab.closable) return; // Don't close non-closable tabs
    
    tabs.value.splice(tabIndex, 1);
    
    // Switch to previous tab or first tab
    if (activeTabId.value === tabId) {
      const newActiveTab = tabs.value[Math.max(0, tabIndex - 1)];
      if (newActiveTab) {
        activeTabId.value = newActiveTab.id;
      } else if (tabs.value.length > 0) {
        activeTabId.value = tabs.value[0].id;
      }
    }
  }
};

const handleIframeError = () => {
  if (activeTab.value?.url?.includes('scrapbox.io')) {
    // Open in WebView window instead
    const projectName = activeTab.value.url.split('/').pop() || 'help-jp';
    openScrapboxProject(projectName);
  }
};

// Scrapbox functions
const openScrapboxHome = async () => {
  const scrapboxUrl = "https://scrapbox.io";
  try {
    await invoke('create_webview_window', { 
      url: scrapboxUrl,
      label: `scrapbox-${Date.now()}`
    });
    
    addToRecent({
      id: `scrapbox-home-${Date.now()}`,
      title: "Scrapbox Home",
      url: scrapboxUrl,
      lastAccessed: new Date()
    });
    
    errorMessage.value = "";
  } catch (error) {
    console.error('Failed to open Scrapbox:', error);
    errorMessage.value = `Scrapboxの起動に失敗しました: ${error}`;
  }
};

const openScrapboxProject = async (projectName: string) => {
  try {
    const url = `https://scrapbox.io/${projectName}`;
    await invoke('create_webview_window', { 
      url,
      label: `scrapbox-${projectName}-${Date.now()}`
    });
    
    addToRecent({
      id: `scrapbox-${projectName}-${Date.now()}`,
      title: `Scrapbox - ${projectName}`,
      url,
      lastAccessed: new Date()
    });
    
    errorMessage.value = "";
  } catch (error) {
    console.error('Failed to open Scrapbox project:', error);
    errorMessage.value = `プロジェクトの起動に失敗しました: ${error}`;
  }
};

const formatDate = (timestamp: number) => {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - timestamp;
  const minutes = Math.floor(diff / 60000);
  
  if (minutes < 1) return "たった今";
  if (minutes < 60) return `${minutes}分前`;
  if (minutes < 1440) return `${Math.floor(minutes / 60)}時間前`;
  return date.toLocaleDateString();
};

// Scrapbox pages functions
const fetchScrapboxPages = async () => {
  scrapboxLoading.value = true;
  scrapboxError.value = '';
  
  try {
    const result = await invoke('fetch_scrapbox_pages', {
      project: scrapboxProject.value,
      skip: scrapboxSkip.value,
      limit: scrapboxLimit.value,
      sort: scrapboxSort.value
    }) as { pages: ScrapboxPage[], count: number, skip: number };
    
    scrapboxPages.value = result.pages;
    console.log(`Fetched ${result.pages.length} pages from ${scrapboxProject.value}`);
  } catch (error) {
    console.error('Failed to fetch Scrapbox pages:', error);
    scrapboxError.value = `ページの取得に失敗しました: ${error}`;
  } finally {
    scrapboxLoading.value = false;
  }
};

const openScrapboxPage = async (page: ScrapboxPage) => {
  try {
    const url = `https://scrapbox.io/${scrapboxProject.value}/${encodeURIComponent(page.title)}`;
    
    // Open in WebView window instead of tab
    await invoke('create_webview_window', { 
      url,
      label: `scrapbox-page-${page.id}-${Date.now()}`
    });
    
    addToRecent({
      id: `scrapbox-page-${page.id}`,
      title: `${page.title} - ${scrapboxProject.value}`,
      url,
      lastAccessed: new Date()
    });
    
    errorMessage.value = "";
  } catch (error) {
    console.error('Failed to open Scrapbox page:', error);
    errorMessage.value = `ページの起動に失敗しました: ${error}`;
  }
};

const changeScrapboxProject = () => {
  scrapboxSkip.value = 0;
  fetchScrapboxPages();
};

// Recent windows functions
const addToRecent = (window: RecentWindow) => {
  // Remove duplicates
  recentWindows.value = recentWindows.value.filter(w => w.id !== window.id);
  // Add to beginning
  recentWindows.value.unshift(window);
  // Keep only 10 items
  if (recentWindows.value.length > 10) {
    recentWindows.value = recentWindows.value.slice(0, 10);
  }
  saveToStorage();
};

const reopenWindow = async (window: RecentWindow) => {
  try {
    const windowId = `reopen-${Date.now()}`;
    await invoke('create_webview_window', { 
      url: window.url,
      label: windowId
    });
    
    // Update recent history
    addToRecent({
      ...window,
      id: windowId,
      lastAccessed: new Date()
    });
    
    errorMessage.value = "";
  } catch (error) {
    console.error('Failed to reopen window:', error);
    errorMessage.value = `ウィンドウの再起動に失敗しました: ${error}`;
  }
};

const removeFromRecent = (windowId: string) => {
  recentWindows.value = recentWindows.value.filter(w => w.id !== windowId);
  saveToStorage();
};

// Favorites functions
const addFavoriteFromWebView = async (url: string, title: string) => {
  try {
    // Check if already exists
    const existingFavorite = favorites.value.find(f => f.url === url);
    if (existingFavorite) {
      errorMessage.value = "すでにお気に入りに登録されています";
      setTimeout(() => {
        errorMessage.value = "";
      }, 2000);
      return;
    }
    
    const favorite: Favorite = {
      id: `fav-${Date.now()}`,
      title,
      url
    };
    
    favorites.value.unshift(favorite);
    saveToStorage();
    errorMessage.value = `お気に入りに追加しました: ${title}`;
    
    setTimeout(() => {
      errorMessage.value = "";
    }, 3000);
  } catch (error) {
    console.error('Failed to add favorite from WebView:', error);
    errorMessage.value = `お気に入りの追加に失敗しました: ${error}`;
  }
};

const openFavorite = async (favorite: Favorite) => {
  try {
    const windowId = `favorite-${Date.now()}`;
    await invoke('create_webview_window', { 
      url: favorite.url,
      label: windowId
    });
    
    addToRecent({
      id: windowId,
      title: favorite.title,
      url: favorite.url,
      lastAccessed: new Date()
    });
    
    errorMessage.value = "";
  } catch (error) {
    console.error('Failed to open favorite:', error);
    errorMessage.value = `お気に入りの起動に失敗しました: ${error}`;
  }
};

const removeFavorite = (favoriteId: string) => {
  favorites.value = favorites.value.filter(f => f.id !== favoriteId);
  saveToStorage();
};

// Utility functions
const formatTime = (date: Date) => {
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  
  if (minutes < 1) return "たった今";
  if (minutes < 60) return `${minutes}分前`;
  if (minutes < 1440) return `${Math.floor(minutes / 60)}時間前`;
  return date.toLocaleDateString();
};

// Data persistence
const saveToStorage = () => {
  localStorage.setItem('sbe-recent', JSON.stringify(recentWindows.value.map(w => ({
    ...w,
    lastAccessed: w.lastAccessed.toISOString()
  }))));
  localStorage.setItem('sbe-favorites', JSON.stringify(favorites.value));
};

const loadFromStorage = () => {
  try {
    const recentData = localStorage.getItem('sbe-recent');
    if (recentData) {
      const parsed = JSON.parse(recentData);
      recentWindows.value = parsed.map((w: any) => ({
        ...w,
        lastAccessed: new Date(w.lastAccessed)
      }));
    }
    
    const favData = localStorage.getItem('sbe-favorites');
    if (favData) {
      favorites.value = JSON.parse(favData);
    }
  } catch (error) {
    console.error('Failed to load from storage:', error);
  }
};

// Navigation tracking
let navigationUnlisten: (() => void) | null = null;

// Initialize
onMounted(async () => {
  loadFromStorage();
  
  // Add sample favorites if none exist
  if (favorites.value.length === 0) {
    favorites.value = [
      {
        id: 'sample-1',
        title: 'Scrapbox ヘルプ',
        url: 'https://scrapbox.io/help-jp'
      }
    ];
    saveToStorage();
  }

  // Listen for navigation events from WebView windows
  navigationUnlisten = await listen('add-to-recent', (event: any) => {
    const { window_label, url, title } = event.payload;
    
    addToRecent({
      id: `${window_label}-${Date.now()}`,
      title: title || new URL(url).hostname,
      url,
      lastAccessed: new Date()
    });

    console.log(`Navigation tracked: ${title} (${url})`);
  });
  
  // Listen for add to favorites events from WebViews
  await listen('add-to-favorites', (event: any) => {
    console.log('📥 Received add-to-favorites event:', event.payload);
    const { url, title } = event.payload;
    addFavoriteFromWebView(url, title);
  });

  // Listen for title updates
  await listen('update-recent-title', (event: any) => {
    const { url, title } = event.payload;
    
    // Find and update existing recent window with same URL
    const existingIndex = recentWindows.value.findIndex(w => w.url === url);
    if (existingIndex >= 0) {
      recentWindows.value[existingIndex].title = title;
      recentWindows.value[existingIndex].lastAccessed = new Date();
      saveToStorage();
      console.log(`Title updated: ${title} (${url})`);
    }
  });
});

// Watch for tab changes to auto-load Scrapbox pages
watch(activeTabId, (newTabId) => {
  if (newTabId === 'scrapbox-pages' && scrapboxPages.value.length === 0) {
    fetchScrapboxPages();
  }
});

onUnmounted(() => {
  if (navigationUnlisten) {
    navigationUnlisten();
  }
});

// Functions are used in template - no need to export in script setup
</script>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.tab-bar {
  display: flex;
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
  padding: 0;
  overflow-x: auto;
}

.tab {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  border-right: 1px solid #ddd;
  cursor: pointer;
  min-width: 120px;
  background: white;
  transition: background-color 0.2s;
}

.tab:hover {
  background: #f0f0f0;
}

.tab.active {
  background: #007acc;
  color: white;
}

.tab-icon {
  margin-right: 8px;
  font-size: 14px;
}

.tab-title {
  flex-grow: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 13px;
}

.close-btn {
  margin-left: 8px;
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  font-size: 16px;
  padding: 0 4px;
  border-radius: 2px;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.add-tab-btn {
  background: #f5f5f5;
  border: none;
  padding: 8px 16px;
  cursor: pointer;
  font-size: 16px;
  color: #666;
}

.add-tab-btn:hover {
  background: #e0e0e0;
}

.error-message {
  background: #ffebee;
  color: #c62828;
  padding: 8px 16px;
  border-bottom: 1px solid #ffcdd2;
  font-size: 14px;
}

.tab-content {
  flex: 1;
  overflow: auto;
}

/* Manager View */
.manager-view {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.scrapbox-home-section {
  margin-bottom: 32px;
  text-align: center;
}

.scrapbox-home-btn {
  display: inline-flex;
  align-items: center;
  gap: 12px;
  padding: 16px 32px;
  background: linear-gradient(135deg, #007acc, #0056b3);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 122, 204, 0.3);
}

.scrapbox-home-btn:hover {
  background: linear-gradient(135deg, #0056b3, #004085);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 122, 204, 0.4);
}

.scrapbox-home-btn:active {
  transform: translateY(0);
  box-shadow: 0 2px 4px rgba(0, 122, 204, 0.3);
}

.btn-icon {
  font-size: 20px;
}

.btn-text {
  font-size: 16px;
}

.section {
  margin-bottom: 32px;
}

.section h2 {
  margin: 0 0 16px 0;
  color: #333;
  font-size: 18px;
}

.favorite-instruction {
  padding: 12px;
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 4px;
  margin-bottom: 16px;
  color: #6c757d;
  font-size: 14px;
  text-align: center;
}

.empty-state {
  text-align: center;
  color: #888;
  padding: 32px;
  font-style: italic;
}

.window-item, .favorite-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  margin-bottom: 8px;
  background: white;
}

.window-info, .favorite-info {
  flex-grow: 1;
}

.window-title, .favorite-title {
  font-weight: 500;
  margin-bottom: 4px;
}

.window-url, .favorite-url {
  color: #666;
  font-size: 13px;
  margin-bottom: 4px;
}

.window-time {
  color: #999;
  font-size: 12px;
}

.window-actions, .favorite-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  background: #007acc;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.action-btn:hover {
  background: #005999;
}

.remove-btn {
  background: #e53e3e;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.remove-btn:hover {
  background: #c53030;
}

.add-favorite {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.url-input {
  flex-grow: 1;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.add-btn {
  background: #38a169;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.add-btn:hover {
  background: #2f855a;
}

.quick-actions {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.action-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 24px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background: white;
  cursor: pointer;
  transition: all 0.2s;
}

.action-card:hover {
  background: #f8f9fa;
  border-color: #007acc;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.action-icon {
  font-size: 32px;
  margin-bottom: 8px;
}

.action-text {
  font-weight: 500;
  color: #333;
}

/* Scrapbox Pages View */
.scrapbox-pages-view {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
}

.scrapbox-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.scrapbox-header h2 {
  margin: 0;
  color: #333;
}

.scrapbox-controls {
  display: flex;
  gap: 12px;
  align-items: center;
}

.project-input {
  padding: 6px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  min-width: 150px;
}

.sort-select {
  padding: 6px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.scrapbox-page-list {
  background: white;
  border-radius: 6px;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.page-header {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1.5fr 1.5fr;
  gap: 16px;
  padding: 12px 16px;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
  font-weight: 600;
  font-size: 14px;
  color: #666;
}

.scrapbox-page-item {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 1.5fr 1.5fr;
  gap: 16px;
  padding: 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: all 0.2s;
}

.scrapbox-page-item:hover {
  background: #f8f9fa;
}

.scrapbox-page-item:last-child {
  border-bottom: none;
}

.scrapbox-page-item .page-title {
  font-weight: 500;
  color: #333;
  margin: 0;
}

.page-views {
  color: #666;
  text-align: right;
}

.page-links {
  color: #666;
  text-align: right;
}

.page-user {
  display: flex;
  align-items: center;
  gap: 8px;
}

.user-name {
  font-size: 12px;
  color: #999;
  font-family: monospace;
}

.loading-state, .empty-state {
  text-align: center;
  padding: 40px;
  color: #666;
}

.error-message {
  background: #fee;
  color: #c33;
  padding: 12px;
  border-radius: 4px;
  margin-bottom: 16px;
  border: 1px solid #fcc;
}

/* WebView */
.webview-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.webview-header {
  display: flex;
  padding: 8px;
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
  gap: 8px;
}

.navigate-btn {
  background: #007acc;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
}

.navigate-btn:hover {
  background: #005999;
}

.webview-iframe {
  flex: 1;
  border: none;
  width: 100%;
}

.loading-indicator {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 64px;
  color: #666;
  font-style: italic;
}

/* Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal {
  background: white;
  padding: 24px;
  border-radius: 8px;
  width: 400px;
  max-width: 90vw;
}

.modal h3 {
  margin: 0 0 16px 0;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 16px;
}

.cancel-btn {
  background: #666;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.cancel-btn:hover {
  background: #555;
}
</style>