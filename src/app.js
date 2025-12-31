// ========================================
// OPEN EXTERNAL LINKS
// ========================================

async function openExternal(url) {
    try {
        await window.__TAURI_INTERNALS__.invoke('open_url', { url });
    } catch (error) {
        console.error('Failed to open URL:', error);
        // Fallback
        window.open(url, '_blank');
    }
}

window.openExternal = openExternal;

// ========================================
// INITIALIZATION
// ========================================

document.addEventListener('DOMContentLoaded', () => {
    console.log('RL Camera Path Editor - Ready!');
});