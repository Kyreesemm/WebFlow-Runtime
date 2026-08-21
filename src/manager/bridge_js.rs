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

pub const DEBUG_INJECTED_JS: &str = r#"
(function() {
    window.__WEBFLOW_DEBUG__ = true;

    function debugToBackend(type, details) {
        if (!window.ipc || !window.ipc.postMessage) return;
        window.ipc.postMessage(JSON.stringify({
            id: 0,
            cmd: '__debug',
            args: [{ type: type, details: details }]
        }));
    }

    function shouldTraceCommand(command) {
        if (window.__WEBFLOW_DEBUG_VERBOSE__) return true;
        return [
            'getRunningApps', 'getTotalCacheSize', 'getTotalDataSize',
            'getAppStorageSizes'
        ].indexOf(command) === -1;
    }

    function describeElement(element) {
        if (!element || !element.tagName) return null;
        return {
            tag: element.tagName.toLowerCase(),
            id: element.id || null,
            name: element.getAttribute('name') || null,
            type: element.getAttribute('type') || null,
            text: (element.innerText || element.textContent || '').trim().slice(0, 120),
            value: typeof element.value === 'string' ? element.value.slice(0, 200) : null,
            checked: typeof element.checked === 'boolean' ? element.checked : null
        };
    }

    document.addEventListener('click', function(event) {
        debugToBackend('ui.click', { element: describeElement(event.target) });
    }, true);
    document.addEventListener('change', function(event) {
        debugToBackend('ui.change', { element: describeElement(event.target) });
    }, true);
    document.addEventListener('submit', function(event) {
        debugToBackend('ui.submit', { element: describeElement(event.target) });
    }, true);
    window.addEventListener('error', function(event) {
        debugToBackend('js.error', {
            message: event.message,
            source: event.filename,
            line: event.lineno,
            column: event.colno
        });
    });
    window.addEventListener('unhandledrejection', function(event) {
        debugToBackend('js.unhandledrejection', { reason: String(event.reason) });
    });
    ['log', 'info', 'warn', 'error'].forEach(function(level) {
        const original = console[level].bind(console);
        console[level] = function(...args) {
            debugToBackend('js.console', {
                level: level,
                messages: args.map(function(value) { return String(value).slice(0, 500); })
            });
            return original(...args);
        };
    });

    const originalQWebChannel = window.QWebChannel;
    if (originalQWebChannel) {
        window.QWebChannel = function(transport, callback) {
            return originalQWebChannel(transport, function(channel) {
                const managerAPI = channel.objects.managerAPI;
                Object.keys(managerAPI).forEach(function(methodName) {
                    if (typeof managerAPI[methodName] !== 'function') return;
                    const originalMethod = managerAPI[methodName];
                    managerAPI[methodName] = function(...args) {
                        const callbackIndex = args.length - 1;
                        const callback = typeof args[callbackIndex] === 'function'
                            ? args[callbackIndex]
                            : null;
                        const requestArgs = callback ? args.slice(0, callbackIndex) : args;
                        if (shouldTraceCommand(methodName)) {
                            debugToBackend('ipc.request', {
                                id: null,
                                command: methodName,
                                args: requestArgs
                            });
                        }
                        if (callback) {
                            args[callbackIndex] = function(data) {
                                if (shouldTraceCommand(methodName)) {
                                    debugToBackend('ipc.response', {
                                        command: methodName,
                                        data: data
                                    });
                                }
                                return callback(data);
                            };
                        }
                        return originalMethod.apply(this, args);
                    };
                });
                callback(channel);
            });
        };
    }
})();
"#;
