pub const INJECTED_BRIDGE_JS: &str = r#"
(function() {
    console.log("[WebFlow] Initializing Rust IPC Bridge...");

    window.__webflow_callbacks = {};
    window.__webflow_cb_id = 0;

    window.__WEBFLOW_IPC_CALLBACK__ = function(id, data) {
        if (window.__webflow_callbacks[id]) {
            try {
                window.__webflow_callbacks[id](data);
            } catch (e) {
                console.error("[WebFlow] Error in IPC callback:", e);
            }
            delete window.__webflow_callbacks[id];
        }
    };

    const managerAPIObj = {
        appsChanged: {
            _listeners: [],
            connect: function(fn) { this._listeners.push(fn); },
            emit: function() { this._listeners.forEach(fn => { try { fn(); } catch(e){} }); }
        }
    };

    function createApiMethod(cmdName) {
        return function(...args) {
            let callback = null;
            if (args.length > 0 && typeof args[args.length - 1] === 'function') {
                callback = args.pop();
            }
            const id = ++window.__webflow_cb_id;
            if (callback) {
                window.__webflow_callbacks[id] = callback;
            }
            const payload = JSON.stringify({
                id: id,
                cmd: cmdName,
                args: args
            });
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(payload);
            } else {
                console.error("[WebFlow] window.ipc.postMessage not available");
            }
        };
    }

    const methods = [
        'listApps', 'getAppConfig', 'createApp', 'updateApp',
        'createAppWithIcon', 'updateAppWithIcon', 'deleteApp',
        'runApp', 'getRunningApps', 'listTemplates', 'createFromTemplate',
        'getEngineSettings', 'updateEngineSettings', 'changeUserdataPath', 'listUserAgents',
        'addUserAgent', 'deleteUserAgent', 'getWindowState', 'saveWindowState',
        'clearAppCache', 'clearAppData', 'clearAllCache', 'clearAllData',
        'getTotalCacheSize', 'getTotalDataSize', 'getAppStorageSizes', 'selectFolder', 'openFolder',
        'listCookieBrowsers', 'importBrowserCookies'
    ];

    methods.forEach(m => {
        managerAPIObj[m] = createApiMethod(m);
    });

    window.managerAPI = managerAPIObj;
    window.qt = { webChannelTransport: {} };
    window.QWebChannel = function(transport, callback) {
        if (callback) {
            setTimeout(() => {
                callback({ objects: { managerAPI: managerAPIObj } });
            }, 0);
        }
    };

    console.log("[WebFlow] Rust IPC Bridge initialized.");
})();
"#;
