let managerAPI = null;
window.currentImportedCookies = [];

// Переводы
const translations = {
    ru: {
        create_app: 'Создать приложение',
        edit_app: 'Редактировать приложение',
        refresh: 'Обновить',
        from_template: 'Из шаблона',
        app_name: 'Название приложения',
        app_url: 'URL сайта',
        app_icon: 'Иконка приложения',
        current_icon: 'Текущая иконка',
        window_title: 'Заголовок окна',
        window_width: 'Ширина окна (px)',
        window_height: 'Высота окна (px)',
        allow_resize: 'Разрешить изменение размера',
        custom_scrollbar: 'Кастомный скроллбар',
        isolated_storage: 'Изолированное хранилище',
        custom_css: 'Кастомный CSS (опционально)',
        custom_js: 'Кастомный JavaScript (опционально)',
        save: 'Сохранить',
        cancel: 'Отмена',
        default: 'По умолчанию',
        custom: 'Свой',
        custom_useragent: 'Свой User-Agent',
        select_template: 'Выбрать шаблон',
        add_useragent: 'Добавить User-Agent',
        name: 'Название',
        useragent_string: 'User-Agent строка',
        add: 'Добавить',
        general_settings: 'Общие параметры',
        autostart: 'Автозапуск менеджера',
        autostart_desc: 'Запускать менеджер при старте системы',
        minimize_tray: 'Минимизация в трей',
        minimize_tray_desc: 'Сворачивать приложения в системный трей',
        app_tray_icons: 'Иконки приложений в трее',
        app_tray_icons_desc: 'Отображать отдельную иконку в системном трее для каждого запущенного приложения',
        tray_apps_menu: 'Меню приложений в трее менеджера',
        tray_apps_menu_desc: 'Показывать список приложений в контекстном меню иконки трея менеджера',
        storage_mode: 'Режим хранилища',
        isolated_storage_desc: 'Каждое приложение использует отдельное хранилище куки и данных',
        data_management: 'Управление данными',
        clear_cache: 'Очистить кеш всех приложений',
        clear_cookies: 'Очистить куки всех приложений',
        total_cache_size: 'Общий размер кэша:',
        total_data_size: 'Общий размер данных:',
        private_storage_size: 'Пользовательские данные:',
        private_cache_size: 'Кэш:',
        private_data_size: 'Данные:',
        private_storage_title: 'Изолированные хранилища приложений',
        no_private_storage: 'Нет приложений с изолированным хранилищем',
        no_apps: 'Приложения не найдены',
        no_apps_desc: 'Создайте своё первое приложение, нажав кнопку "Создать приложение"',
        status_idle: 'Не запущено',
        status_running: 'Запущено',
        status_editing: 'Редактируется',
        app_settings: 'Настройки приложения',
        edit_app: 'Редактировать приложение',
        delete_app: 'Удалить приложение',
        clear_app_cache: 'Очистить кэш',
        clear_app_data: 'Очистить данные',
        delete_confirm: 'Вы уверены, что хотите удалить это приложение?',
        delete_ua_confirm: 'Удалить этот User-Agent?',
        engine_settings_title: 'Настройки движка',
        userdata_path: 'Путь к пользовательским данным',
        userdata_path_hint: 'Здесь хранятся все приложения и их данные',
        select_folder: 'Выбрать',
        open_folder: 'Открыть',
        current_paths: 'Текущие пути',
        apps_path: 'Приложения',
        config_path: 'Настройки',
        runtime_path: 'Runtime',
        shared_storage_path: 'Общее хранилище',
        save_settings: 'Сохранить настройки',
        reload_settings: 'Обновить',
        settings_saved: 'Настройки сохранены',
        error_saving: 'Ошибка сохранения',
        userdata_change_title: 'Смена папки userdata',
        userdata_change_message: 'Вы уверены, что хотите сменить папку пользовательских данных?',
        userdata_transfer: 'Перенести пользовательские данные в новую папку',
        userdata_delete_old: 'Удалить старую папку',
        userdata_changed: 'Папка пользовательских данных изменена',
        confirm_title: 'Подтверждение',
        ok: 'OK',
        delete: 'Удалить',
        clear: 'Очистить',
        cache_cleared: 'Кэш очищен',
        data_cleared: 'Данные очищены',
        all_cache_cleared: 'Кэш всех приложений очищен',
        all_data_cleared: 'Данные всех приложений очищены',
        clear_all_cache_confirm: 'Очистить кэш всех приложений?',
        clear_all_data_confirm: 'Очистить данные всех приложений?',
        import_browser_cookies: 'Импорт cookies из браузера',
        import_browser_cookies_desc: 'Импортировать cookies из браузера в хранилище приложения',
        browser_source: 'Браузер-источник',
        browser_profile: 'Профиль браузера',
        import_now: 'Импортировать',
        cookies_imported: 'Cookies импортированы',
        import_failed: 'Ошибка импорта',
        choose_browser: 'Выберите браузер',
        default_profile: 'Профиль по умолчанию',
        choose_file: 'Выберите файл',
        no_file_selected: 'Файл не выбран',
        no_importable_cookies: 'Подходящие cookies не найдены',
        google_oauth_fallback: 'Обход блокировок Google OAuth',
        google_oauth_fallback_desc: 'Использовать системный браузер для авторизации через Google аккаунт'
    },
    en: {
        create_app: 'Create Application',
        edit_app: 'Edit Application',
        refresh: 'Refresh',
        from_template: 'From Template',
        app_name: 'Application Name',
        app_url: 'Website URL',
        app_icon: 'Application Icon',
        current_icon: 'Current icon',
        window_title: 'Window Title',
        window_width: 'Window Width (px)',
        window_height: 'Window Height (px)',
        allow_resize: 'Allow Resizing',
        custom_scrollbar: 'Custom Scrollbar',
        isolated_storage: 'Isolated Storage',
        custom_css: 'Custom CSS (optional)',
        custom_js: 'Custom JavaScript (optional)',
        save: 'Save',
        cancel: 'Cancel',
        default: 'Default',
        custom: 'Custom',
        custom_useragent: 'Custom User-Agent',
        select_template: 'Select Template',
        add_useragent: 'Add User-Agent',
        name: 'Name',
        useragent_string: 'User-Agent String',
        add: 'Add',
        general_settings: 'General Settings',
        autostart: 'Manager Autostart',
        autostart_desc: 'Start manager on system startup',
        minimize_tray: 'Minimize to Tray',
        minimize_tray_desc: 'Minimize applications to system tray',
        app_tray_icons: 'Application Tray Icons',
        app_tray_icons_desc: 'Show a separate system tray icon for each running application',
        tray_apps_menu: 'Manager Tray Apps Menu',
        tray_apps_menu_desc: 'Show list of applications in manager tray icon context menu',
        storage_mode: 'Storage Mode',
        isolated_storage_desc: 'Each application uses separate cookie and data storage',
        data_management: 'Data Management',
        clear_cache: 'Clear Cache for All Apps',
        clear_cookies: 'Clear Cookies for All Apps',
        total_cache_size: 'Total Cache Size:',
        total_data_size: 'Total Data Size:',
        private_storage_size: 'User Data:',
        private_cache_size: 'Cache:',
        private_data_size: 'Data:',
        private_storage_title: 'Isolated Application Storage',
        no_private_storage: 'No applications use isolated storage',
        no_apps: 'No Applications Found',
        no_apps_desc: 'Create your first application by clicking "Create Application"',
        status_idle: 'Idle',
        status_running: 'Running',
        status_editing: 'Editing',
        file_not_selected: 'No file chosen',
        choose_file: 'Choose File',
        app_settings: 'Application Settings',
        edit_app: 'Edit Application',
        delete_app: 'Delete Application',
        clear_app_cache: 'Clear Cache',
        clear_app_data: 'Clear Data',
        delete_confirm: 'Are you sure you want to delete this application?',
        delete_ua_confirm: 'Delete this User-Agent?',
        engine_settings_title: 'Engine Settings',
        userdata_path: 'User Data Path',
        userdata_path_hint: 'All applications and their data are stored here',
        select_folder: 'Select',
        open_folder: 'Open',
        current_paths: 'Current Paths',
        apps_path: 'Applications',
        config_path: 'Settings',
        runtime_path: 'Runtime',
        shared_storage_path: 'Shared Storage',
        save_settings: 'Save Settings',
        reload_settings: 'Refresh',
        settings_saved: 'Settings saved',
        error_saving: 'Error saving settings',
        userdata_change_title: 'Change userdata folder',
        userdata_change_message: 'Are you sure you want to change the user data folder?',
        userdata_transfer: 'Transfer user data to the new folder',
        userdata_delete_old: 'Delete the old folder',
        userdata_changed: 'User data folder changed',
        confirm_title: 'Confirmation',
        ok: 'OK',
        delete: 'Delete',
        clear: 'Clear',
        cache_cleared: 'Cache cleared',
        data_cleared: 'Data cleared',
        all_cache_cleared: 'Cache cleared for all applications',
        all_data_cleared: 'Data cleared for all applications',
        clear_all_cache_confirm: 'Clear cache for all applications?',
        clear_all_data_confirm: 'Clear data for all applications?',
        import_browser_cookies: 'Import browser cookies',
        import_browser_cookies_desc: 'Import cookies from a browser into application storage',
        browser_source: 'Browser source',
        browser_profile: 'Browser profile',
        import_now: 'Import',
        cookies_imported: 'Cookies imported',
        import_failed: 'Import failed',
        choose_browser: 'Select browser',
        default_profile: 'Default profile',
        choose_file: 'Choose file',
        no_file_selected: 'No file selected',
        no_importable_cookies: 'No importable cookies were found',
        google_oauth_fallback: 'Google OAuth Fallback',
        google_oauth_fallback_desc: 'Use system default browser for Google account login'
    }
};

// Определение языка системы
function getSystemLanguage() {
    const lang = (navigator.language || navigator.userLanguage || '').toLowerCase();
    return lang.startsWith('ru') ? 'ru' : 'en';
}

// Текущий язык
const savedLanguage = localStorage.getItem('language');
let currentLang = translations[savedLanguage] ? savedLanguage : getSystemLanguage();
document.body.setAttribute('data-lang', currentLang);

// Применение переводов
function applyTranslations() {
    document.documentElement.lang = currentLang;
    document.querySelectorAll('[data-i18n]').forEach(element => {
        const key = element.getAttribute('data-i18n');
        if (translations[currentLang][key]) {
            if (element.tagName === 'INPUT' || element.tagName === 'TEXTAREA') {
                element.placeholder = translations[currentLang][key];
            } else {
                element.textContent = translations[currentLang][key];
            }
        }
    });

    document.querySelectorAll('[data-i18n-title]').forEach(element => {
        const key = element.getAttribute('data-i18n-title');
        if (translations[currentLang][key]) {
            element.setAttribute('title', translations[currentLang][key]);
        }
    });

    populateCookieBrowserOptions();

    // Обновить tooltips для вкладок
    document.querySelectorAll('.tab-icon').forEach(tab => {
        const tabName = tab.getAttribute('data-tab');
        const tooltips = {
            apps: currentLang === 'ru' ? 'Приложения' : 'Applications',
            settings: currentLang === 'ru' ? 'Общие настройки' : 'General Settings',
            storage: currentLang === 'ru' ? 'Хранилище' : 'Storage',
            useragents: currentLang === 'ru' ? 'User-Agent\'ы' : 'User-Agents',
            engine: currentLang === 'ru' ? 'Настройки движка' : 'Engine Settings'
        };
        if (tooltips[tabName]) {
            tab.setAttribute('title', tooltips[tabName]);
        }
    });

    // Обновить tooltips для кнопок
    const themeToggle = document.querySelector('.theme-toggle');
    if (themeToggle) {
        themeToggle.setAttribute('title', currentLang === 'ru' ? 'Тема' : 'Theme');
    }

    const langToggle = document.querySelector('.lang-toggle');
    if (langToggle) {
        langToggle.setAttribute('title', currentLang === 'ru' ? 'Язык' : 'Language');
    }

    // Обновить текст языка
    const langText = document.querySelector('.lang-text');
    if (langText) {
        langText.textContent = currentLang.toUpperCase();
    }

    // Обновить file input (это делается через CSS, но текст нужно обновить через JS)
    // Браузер не позволяет менять текст file input напрямую, это ограничение безопасности
    updateFileInputLabel();
}

// Заполняем выбор User-Agent тем же встроенным списком, который показывает
// вкладка User-Agent'ов. Пользовательские записи намеренно не добавляются.
function populateBuiltInUserAgents(selectedId = 'default') {
    if (!managerAPI || !managerAPI.listUserAgents) return;

    managerAPI.listUserAgents(function(uaJson) {
        try {
            const userAgents = JSON.parse(uaJson).filter(ua => !ua.custom);
            const select = document.getElementById('user-agent');
            if (!select) return;

            select.innerHTML = userAgents.map(ua =>
                `<option value="${ua.id}">${ua.id === 'default' ? 'Default' : ua.name}</option>`
            ).join('');

            select.value = userAgents.some(ua => ua.id === selectedId)
                ? selectedId
                : 'default';
        } catch (error) {
            console.error('Error loading built-in User-Agents:', error);
        }
    });
}

// Переключение языка
function toggleLanguage() {
    currentLang = currentLang === 'ru' ? 'en' : 'ru';
    document.body.setAttribute('data-lang', currentLang);
    localStorage.setItem('language', currentLang);
    applyTranslations();
    saveFullWindowState();
    loadApps(); // Перезагрузить приложения для обновления текста
}

// Переключение темы с анимацией
function toggleTheme() {
    const body = document.body;
    body.classList.add('theme-transitioning');

    const currentTheme = body.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    body.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);

    setTimeout(() => {
        body.classList.remove('theme-transitioning');
    }, 500);

    saveFullWindowState();
}

// Сохранение полного состояния окна (тема, язык, активная вкладка)
function saveFullWindowState() {
    if (!managerAPI || !managerAPI.saveWindowState) return;

    const activeTabElem = document.querySelector('.tab-icon.active');
    const activeTab = activeTabElem ? activeTabElem.getAttribute('data-tab') : 'apps';

    const state = {
        theme: document.body.getAttribute('data-theme') || 'dark',
        language: currentLang,
        active_tab: activeTab
    };

    managerAPI.saveWindowState(JSON.stringify(state));
}

// Инициализация Qt WebChannel
new QWebChannel(qt.webChannelTransport, function(channel) {
    managerAPI = channel.objects.managerAPI;
    console.log('WebChannel connected');
    populateBuiltInUserAgents();

    // Подписываемся на сигнал изменения списка приложений
    if (managerAPI.appsChanged) {
        managerAPI.appsChanged.connect(function() {
            loadApps();
            updateStorageInfo();
        });
    }

    // Восстанавливаем состояние темы, языка и вкладки из backend
    if (managerAPI.getWindowState) {
        managerAPI.getWindowState(function(stateJson) {
            try {
                const state = JSON.parse(stateJson);
                if (state.theme) {
                    document.body.setAttribute('data-theme', state.theme);
                    localStorage.setItem('theme', state.theme);
                }
                if (state.language) {
                    currentLang = state.language;
                    document.body.setAttribute('data-lang', currentLang);
                    localStorage.setItem('language', currentLang);
                }
                if (state.active_tab) {
                    const tabBtn = document.querySelector(`.tab-icon[data-tab="${state.active_tab}"]`);
                    if (tabBtn) {
                        document.querySelectorAll('.tab-icon').forEach(t => t.classList.remove('active'));
                        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
                        tabBtn.classList.add('active');
                        document.getElementById('tab-' + state.active_tab)?.classList.add('active');

                        // Если восстанавливаем вкладку settings, engine или storage, загружаем соответствующие данные
                        if (state.active_tab === 'settings' || state.active_tab === 'engine') {
                            setTimeout(() => loadEngineSettings(), 50);
                        } else if (state.active_tab === 'storage') {
                            setTimeout(() => startStorageUpdater(), 50);
                        }
                    }
                }
            } catch (e) {
                console.error('Error parsing window state:', e);
            }

            applyTranslations();
            loadApps();
            loadUserAgents();
            startStorageUpdater();
            loadAvailableCookieBrowsers();
            startStatusUpdater();
        });
    } else {
        applyTranslations();
        populateBuiltInUserAgents();
        loadApps();
        loadUserAgents();
        startStorageUpdater();
        loadAvailableCookieBrowsers();
        startStatusUpdater();
    }
});

// Загрузка сохранённой темы
const savedTheme = localStorage.getItem('theme') || 'dark';
document.body.setAttribute('data-theme', savedTheme);

// Управление вкладками (иконки в header)
document.addEventListener('DOMContentLoaded', function() {
    document.querySelectorAll('.tab-icon').forEach(tab => {
        tab.addEventListener('click', function() {
            const tabName = this.getAttribute('data-tab');

            // Убираем активность со всех вкладок
            document.querySelectorAll('.tab-icon').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));

            // Активируем выбранную вкладку
            this.classList.add('active');
            document.getElementById('tab-' + tabName).classList.add('active');

            // Сохраняем состояние окна и вкладок
            saveFullWindowState();

            // Загружаем настройки движка или данные хранилища при переключении вкладок
            if (tabName === 'engine' || tabName === 'settings') {
                loadEngineSettings();
            } else if (tabName === 'storage') {
                startStorageUpdater();
            } else {
                stopStorageUpdater();
            }
        });
    });
});

// Показать/скрыть поле кастомного User-Agent
document.getElementById('cookie-browser')?.addEventListener('change', updateCookieProfileOptions);

function updateFileInputLabel() {
    const input = document.getElementById('app-icon');
    if (!input) return;

    const button = document.getElementById('app-icon-button');
    const fileName = document.getElementById('app-icon-filename');

    if (button) {
        button.textContent = translations[currentLang].choose_file;
    }

    if (fileName && !input.files?.length) {
        fileName.textContent = translations[currentLang].no_file_selected;
    }
}

// Превью иконки
document.getElementById('app-icon').addEventListener('change', function(e) {
    const file = e.target.files[0];
    const fileName = document.getElementById('app-icon-filename');

    if (fileName) {
        fileName.textContent = file ? file.name : translations[currentLang].no_file_selected;
    }

    if (file) {
        const reader = new FileReader();
        reader.onload = function(event) {
            const preview = document.getElementById('icon-preview');
            const img = document.getElementById('icon-preview-img');
            img.src = event.target.result;
            preview.style.display = 'flex';
        };
        reader.readAsDataURL(file);
    }
});

// Progress Bar
function showProgress() {
    document.getElementById('progress-container').classList.add('active');
    document.getElementById('progress-bar').style.width = '0%';

    let progress = 0;
    const interval = setInterval(() => {
        progress += 10;
        document.getElementById('progress-bar').style.width = progress + '%';
        if (progress >= 100) {
            clearInterval(interval);
            setTimeout(hideProgress, 300);
        }
    }, 100);
}

function hideProgress() {
    document.getElementById('progress-container').classList.remove('active');
}

let confirmResolver = null;
let userdataChangeResolver = null;

function showNotification(message, type = 'success', duration = 3200) {
    const container = document.getElementById('notification-container');
    if (!container) return;

    const notification = document.createElement('div');
    notification.className = `notification ${type}`;
    notification.textContent = message;
    container.appendChild(notification);

    requestAnimationFrame(() => notification.classList.add('show'));

    setTimeout(() => {
        notification.classList.remove('show');
        setTimeout(() => notification.remove(), 250);
    }, duration);
}

function showConfirm(message, acceptLabel = null) {
    const modal = document.getElementById('confirm-modal');
    document.getElementById('confirm-message').textContent = message;
    document.getElementById('confirm-accept-btn').textContent = acceptLabel || translations[currentLang].ok;
    modal.classList.add('active');

    return new Promise(resolve => {
        confirmResolver = resolve;
    });
}

function closeConfirmModal(accepted) {
    const modal = document.getElementById('confirm-modal');
    modal.classList.remove('active', 'closing');

    if (confirmResolver) {
        const resolver = confirmResolver;
        confirmResolver = null;
        resolver(Boolean(accepted));
    }
}

function showUserdataChangeModal() {
    const modal = document.getElementById('userdata-change-modal');
    document.getElementById('userdata-transfer').checked = true;
    document.getElementById('userdata-delete-old').checked = false;
    modal.classList.add('active');
    return new Promise(resolve => { userdataChangeResolver = resolve; });
}

function closeUserdataChangeModal(accepted) {
    document.getElementById('userdata-change-modal').classList.remove('active', 'closing');
    if (userdataChangeResolver) {
        const resolver = userdataChangeResolver;
        userdataChangeResolver = null;
        resolver(accepted ? {
            transferData: document.getElementById('userdata-transfer').checked,
            deleteOld: document.getElementById('userdata-delete-old').checked
        } : null);
    }
}

let availableCookieBrowsers = [];

function populateCookieBrowserOptions() {
    const browserSelect = document.getElementById('cookie-browser');
    const profileSelect = document.getElementById('cookie-profile');

    if (!browserSelect || !profileSelect) return;

    if (!availableCookieBrowsers.length) {
        browserSelect.innerHTML = `<option value="">${translations[currentLang].choose_browser}</option>`;
        profileSelect.innerHTML = `<option value="">${translations[currentLang].default_profile}</option>`;
        return;
    }

    browserSelect.innerHTML = availableCookieBrowsers.map(browser =>
        `<option value="${browser.id}">${browser.name}</option>`
    ).join('');

    updateCookieProfileOptions();
}

function updateCookieProfileOptions() {
    const browserSelect = document.getElementById('cookie-browser');
    const profileSelect = document.getElementById('cookie-profile');
    if (!browserSelect || !profileSelect) return;

    const browser = availableCookieBrowsers.find(item => item.id === browserSelect.value);
    const profiles = browser?.profiles || [];

    if (!profiles.length) {
        profileSelect.innerHTML = `<option value="Default">${translations[currentLang].default_profile}</option>`;
        return;
    }

    profileSelect.innerHTML = profiles.map(profile =>
        `<option value="${profile.id}">${profile.name}</option>`
    ).join('');
}

// Загрузка списка приложений
function loadApps() {
    if (!managerAPI) {
        console.log('API not ready yet');
        return;
    }

    showProgress();

    managerAPI.listApps(function(appsJson) {
        const apps = JSON.parse(appsJson);
        const container = document.getElementById('apps-container');

        if (apps.length === 0) {
            container.innerHTML = `
                <div class="empty-state">
                    <h2>${translations[currentLang].no_apps}</h2>
                    <p>${translations[currentLang].no_apps_desc}</p>
                </div>
            `;
            return;
        }

        container.innerHTML = apps.map(app => {
            const iconHtml = app.hasIcon && app.iconData
                ? `<img src="${app.iconData}" alt="${app.name}">`
                : `<span>${app.name.charAt(0).toUpperCase()}</span>`;

            // Убираем https:// и http:// из URL
            const displayUrl = app.url.replace(/^https?:\/\//, '');

            return `
                <div class="app-card">
                    <div class="app-card-header">
                        <div class="app-icon">${iconHtml}</div>
                        <div class="app-info">
                            <h3>${app.name}</h3>
                            <p>${displayUrl}</p>
                        </div>
                    </div>
                    <div class="app-status">${translations[currentLang].status_idle}</div>
                    <div class="app-actions">
                        <button class="btn-run" onclick="runApp('${app.id}')" title="${translations[currentLang].create_app.replace('Создать', 'Запустить').replace('Create', 'Run')}">
                            <span class="material-symbols-rounded">play_arrow</span>
                        </button>
                        <button class="btn-settings" onclick="openAppSettings('${app.id}')" title="${translations[currentLang].app_settings}">
                            <span class="material-symbols-rounded">settings</span>
                        </button>
                    </div>
                </div>
            `;
        }).join('');

        updateStorageInfo();
    });
}

// Запустить приложение
function runApp(appId) {
    if (managerAPI) {
        managerAPI.runApp(appId);
    }
}

// Редактировать приложение
function editApp(appId) {
    if (!managerAPI) return;

    managerAPI.getAppConfig(appId, function(configJson) {
        const config = JSON.parse(configJson);

        document.getElementById('modal-title').textContent = translations[currentLang].edit_app;
        document.getElementById('app-id').value = appId;
        document.getElementById('app-name').value = config.name;
        document.getElementById('app-url').value = config.url;
        document.getElementById('window-title').value = config.window.title;
        document.getElementById('window-width').value = config.window.width;
        document.getElementById('window-height').value = config.window.height;
        document.getElementById('window-resizable').checked = config.window.resizable;
        document.getElementById('custom-scrollbar').checked = config.custom_scrollbar;
        document.getElementById('isolated-storage').checked = config.isolated_storage;
        populateBuiltInUserAgents(config.user_agent);
        document.getElementById('custom-css').value = config.custom_css || '';
        document.getElementById('custom-js').value = config.custom_js || '';
        window.currentImportedCookies = config.imported_cookies || [];
        const iconInput = document.getElementById('app-icon');
        if (iconInput) iconInput.value = '';
        const iconPreview = document.getElementById('icon-preview');
        if (iconPreview) iconPreview.style.display = 'none';
        const iconPreviewImage = document.getElementById('icon-preview-img');
        if (iconPreviewImage) iconPreviewImage.removeAttribute('src');
        updateFileInputLabel();

        if (config.icon && config.iconData) {
            if (iconPreviewImage) iconPreviewImage.src = config.iconData;
            if (iconPreview) iconPreview.style.display = 'flex';
            const iconFileName = document.getElementById('app-icon-filename');
            if (iconFileName) iconFileName.textContent = translations[currentLang].current_icon;
        }

        document.getElementById('modal').classList.add('active');
    });
}

// Удалить приложение (теперь через меню настроек)
async function deleteApp(appId) {
    if (!managerAPI) return;

    const confirmed = await showConfirm(translations[currentLang].delete_confirm, translations[currentLang].delete);
    if (!confirmed) return;

    showProgress();
    managerAPI.deleteApp(appId);
    setTimeout(loadApps, 500);
}

// Показать модальное окно создания
function showCreateModal() {
    document.getElementById('modal-title').textContent = translations[currentLang].create_app;
    document.getElementById('app-form').reset();
    document.getElementById('app-id').value = '';
    document.getElementById('icon-preview').style.display = 'none';
    window.currentImportedCookies = [];
    updateFileInputLabel();
    document.getElementById('modal').classList.add('active');
}

// Закрыть модальное окно
function closeModal() {
    const modal = document.getElementById('modal');
    modal.classList.remove('active', 'closing');
}

// Показать модальное окно шаблонов
function showTemplatesModal() {
    if (!managerAPI) return;

    managerAPI.listTemplates(function(templatesJson) {
        const templates = JSON.parse(templatesJson);
        const container = document.getElementById('templates-list');

        container.innerHTML = templates.map(template => {
            // Поддержка как старого формата (строка), так и нового (объект с переводами)
            const name = typeof template.name === 'object'
                ? (template.name[currentLang] || template.name.en || template.name.ru)
                : template.name;

            const description = typeof template.description === 'object'
                ? (template.description[currentLang] || template.description.en || template.description.ru)
                : template.description;

            // Иконка шаблона
            const iconHtml = template.icon
                ? `<img src="file://${template.iconPath}" alt="${name}">`
                : `<span>${name.charAt(0).toUpperCase()}</span>`;

            return `
                <div class="app-card" onclick="createFromTemplate('${template.id}')">
                    <div class="app-card-header">
                        <div class="app-icon">${iconHtml}</div>
                        <div class="app-info">
                            <h3>${name}</h3>
                            <p>${description}</p>
                        </div>
                    </div>
                </div>
            `;
        }).join('');

        document.getElementById('templates-modal').classList.add('active');
    });
}

function closeTemplatesModal() {
    const modal = document.getElementById('templates-modal');
    modal.classList.add('closing');
    setTimeout(() => {
        modal.classList.remove('active', 'closing');
    }, 300);
}

function createFromTemplate(templateId) {
    if (!managerAPI) return;

    showProgress();
    managerAPI.createFromTemplate(templateId);
    closeTemplatesModal();
    setTimeout(loadApps, 500);
}

// Обработка формы приложения
document.getElementById('app-form').addEventListener('submit', function(e) {
    e.preventDefault();

    if (!managerAPI) return;

    showProgress();

    const appName = document.getElementById('app-name').value;
    const appId = document.getElementById('app-id').value || appName.toLowerCase().replace(/\s+/g, '-');

        const config = {
            name: appName,
            url: document.getElementById('app-url').value,
            window: {
                title: document.getElementById('window-title').value || appName,
                width: parseInt(document.getElementById('window-width').value),
                height: parseInt(document.getElementById('window-height').value),
                resizable: document.getElementById('window-resizable').checked,
                custom_frame: false
            },
            user_agent: document.getElementById('user-agent').value,
            custom_user_agent: null,
            custom_scrollbar: document.getElementById('custom-scrollbar').checked,
            isolated_storage: document.getElementById('isolated-storage').checked,
            custom_css: document.getElementById('custom-css').value,
            custom_js: document.getElementById('custom-js').value,
            imported_cookies: window.currentImportedCookies || []
        };


    const configJson = JSON.stringify(config);

    // Обработка иконки
    const iconFile = document.getElementById('app-icon').files[0];
    if (iconFile) {
        const reader = new FileReader();
        reader.onload = function(event) {
            const iconData = event.target.result;

            if (document.getElementById('app-id').value) {
                managerAPI.updateAppWithIcon(appId, configJson, iconData);
            } else {
                managerAPI.createAppWithIcon(appId, configJson, iconData);
            }

            closeModal();
            setTimeout(loadApps, 500);
        };
        reader.readAsDataURL(iconFile);
    } else {
        if (document.getElementById('app-id').value) {
            managerAPI.updateApp(appId, configJson);
        } else {
            managerAPI.createApp(appId, configJson);
        }

        closeModal();
        setTimeout(loadApps, 500);
    }
});

// User Agents
function loadUserAgents() {
    if (!managerAPI) return;

    managerAPI.listUserAgents(function(uaJson) {
        const userAgents = JSON.parse(uaJson);
        const container = document.getElementById('ua-list');

        container.innerHTML = userAgents.map(ua => `
            <div class="ua-item">
                <div class="ua-content">
                    <div class="ua-name">${ua.name}</div>
                    <div class="ua-string">${ua.string}</div>
                </div>
                <div class="ua-actions">
                    ${ua.custom ? `<button class="btn btn-small btn-delete" onclick="deleteUserAgent('${ua.id}')">Удалить</button>` : ''}
                </div>
            </div>
        `).join('');
    });
}

function showAddUAModal() {
    document.getElementById('ua-form').reset();
    document.getElementById('ua-modal').classList.add('active');
}

function closeUAModal() {
    const modal = document.getElementById('ua-modal');
    modal.classList.add('closing');
    setTimeout(() => {
        modal.classList.remove('active', 'closing');
    }, 300);
}

document.getElementById('ua-form').addEventListener('submit', function(e) {
    e.preventDefault();

    if (!managerAPI) return;

    const name = document.getElementById('ua-name').value;
    const string = document.getElementById('ua-string').value;

    managerAPI.addUserAgent(name, string);
    closeUAModal();
    setTimeout(loadUserAgents, 300);
});

async function deleteUserAgent(uaId) {
    if (!managerAPI) return;

    const confirmed = await showConfirm(translations[currentLang].delete_ua_confirm, translations[currentLang].delete);
    if (!confirmed) return;

    managerAPI.deleteUserAgent(uaId);
    setTimeout(loadUserAgents, 300);
}

// Закрытие модальных окон по клику вне их
document.querySelectorAll('.modal').forEach(modal => {
    modal.addEventListener('click', function(e) {
        if (e.target === this) {
            if (this.id === 'userdata-change-modal') {
                closeUserdataChangeModal(false);
            } else if (this.id === 'confirm-modal') {
                closeConfirmModal(false);
            } else {
                this.classList.remove('active');
            }
        }
    });
});

// Модальное окно настроек приложения
let currentAppId = null;

function openAppSettings(appId) {
    currentAppId = appId;
    document.getElementById('app-settings-modal').classList.add('active');
}

function closeAppSettingsModal() {
    const modal = document.getElementById('app-settings-modal');
    modal.classList.remove('active', 'closing');
    currentAppId = null;
}

function editAppFromSettings() {
    const appId = currentAppId;
    closeAppSettingsModal();
    if (appId) {
        editApp(appId);
    }
}

function deleteAppFromSettings() {
    const appId = currentAppId;
    closeAppSettingsModal();
    if (appId) {
        deleteApp(appId);
    }
}

function showImportCookiesModal() {
    if (!currentAppId) return;
    closeAppSettingsModal();
    document.getElementById('import-cookies-modal').classList.add('active');
}

function closeImportCookiesModal() {
    const modal = document.getElementById('import-cookies-modal');
    modal.classList.add('closing');
    setTimeout(() => {
        modal.classList.remove('active', 'closing');
    }, 300);
}

function loadAvailableCookieBrowsers() {
    if (!managerAPI || !managerAPI.listCookieBrowsers) return;

    managerAPI.listCookieBrowsers(function(resultJson) {
        availableCookieBrowsers = JSON.parse(resultJson);
        populateCookieBrowserOptions();
    });
}

async function importCookiesForCurrentApp() {
    if (!managerAPI || !currentAppId) return;

    const browserId = document.getElementById('cookie-browser').value;
    const profileId = document.getElementById('cookie-profile').value;

    if (!browserId) {
        showNotification(translations[currentLang].choose_browser, 'error');
        return;
    }

    showProgress();
    managerAPI.importBrowserCookies(currentAppId, browserId, profileId, function(resultJson) {
        hideProgress();
        const result = JSON.parse(resultJson);

        if (result.success) {
            if (result.imported > 0) {
                const extraInfo = result.skippedEncrypted
                    ? ` (${result.imported} ok, ${result.skippedEncrypted} skipped)`
                    : `: ${result.imported}`;
                showNotification(`${translations[currentLang].cookies_imported}${extraInfo}`, 'success', 5000);
            } else {
                showNotification(translations[currentLang].no_importable_cookies, 'error', 5000);
            }
            closeImportCookiesModal();
        } else {
            showNotification(`${translations[currentLang].import_failed}: ${result.error}`, 'error', 5000);
        }
    });
}

function clearAppCache() {
    const appId = currentAppId;
    if (appId && managerAPI) {
        managerAPI.clearAppCache(appId, result => {
            closeAppSettingsModal();
            updateStorageInfo();
            showNotification(
                result === 'true' ? translations[currentLang].cache_cleared : translations[currentLang].error_saving,
                result === 'true' ? 'success' : 'error'
            );
        });
    }
}

function clearAppData() {
    const appId = currentAppId;
    if (appId && managerAPI) {
        managerAPI.clearAppData(appId, result => {
            closeAppSettingsModal();
            updateStorageInfo();
            showNotification(
                result === 'true' ? translations[currentLang].data_cleared : translations[currentLang].error_saving,
                result === 'true' ? 'success' : 'error'
            );
        });
    }
}

async function clearAllCache() {
    if (!managerAPI) return;

    const confirmed = await showConfirm(translations[currentLang].clear_all_cache_confirm, translations[currentLang].clear);
    if (!confirmed) return;

    showProgress();
    managerAPI.clearAllCache(result => {
        hideProgress();
        updateStorageInfo();
        showNotification(
            result === 'true' ? translations[currentLang].all_cache_cleared : translations[currentLang].error_saving,
            result === 'true' ? 'success' : 'error'
        );
    });
}

async function clearAllData() {
    if (!managerAPI) return;

    const confirmed = await showConfirm(translations[currentLang].clear_all_data_confirm, translations[currentLang].clear);
    if (!confirmed) return;

    showProgress();
    managerAPI.clearAllData(result => {
        hideProgress();
        updateStorageInfo();
        showNotification(
            result === 'true' ? translations[currentLang].all_data_cleared : translations[currentLang].error_saving,
            result === 'true' ? 'success' : 'error'
        );
    });
}

// Обновление статусов приложений
let statusUpdateInterval = null;
let storageUpdateInterval = null;
let storageUpdateInProgress = false;

function startStorageUpdater() {
    if (storageUpdateInterval) clearInterval(storageUpdateInterval);
    updateStorageInfo();
    storageUpdateInterval = setInterval(updateStorageInfo, 2000);
}

function stopStorageUpdater() {
    if (storageUpdateInterval) {
        clearInterval(storageUpdateInterval);
        storageUpdateInterval = null;
    }
}

function startStatusUpdater() {
    // Обновлять статусы каждые 2 секунды
    if (statusUpdateInterval) {
        clearInterval(statusUpdateInterval);
    }

    updateAppStatuses();
    statusUpdateInterval = setInterval(updateAppStatuses, 2000);
}

function stopStatusUpdater() {
    if (statusUpdateInterval) {
        clearInterval(statusUpdateInterval);
        statusUpdateInterval = null;
    }
}

function updateAppStatuses() {
    if (!managerAPI) return;

    managerAPI.getRunningApps(function(runningJson) {
        const runningApps = JSON.parse(runningJson);

        // Обновить статусы всех карточек
        document.querySelectorAll('.app-card').forEach(card => {
            const runButton = card.querySelector('.btn-run');
            if (!runButton) return;

            // Извлечь app_id из onclick атрибута
            const onclickAttr = runButton.getAttribute('onclick');
            const match = onclickAttr.match(/runApp\('([^']+)'\)/);
            if (!match) return;

            const appId = match[1];
            const statusElement = card.querySelector('.app-status');

            if (runningApps.includes(appId)) {
                statusElement.textContent = translations[currentLang].status_running;
                statusElement.classList.add('status-running');
                statusElement.classList.remove('status-idle');
            } else {
                statusElement.textContent = translations[currentLang].status_idle;
                statusElement.classList.add('status-idle');
                statusElement.classList.remove('status-running');
            }
        });
    });
}

// ============================================
// Engine Settings Functions
// ============================================

async function loadEngineSettings() {
    try {
        // Проверяем, инициализирован ли API
        if (!managerAPI) {
            console.warn('API not initialized yet, retrying...');
            setTimeout(loadEngineSettings, 100);
            return;
        }

        const settingsJson = await new Promise((resolve, reject) => {
            managerAPI.getEngineSettings(function(result) {
                resolve(result);
            });
        });

        const settings = JSON.parse(settingsJson);

        // Заполняем поля
        const autostartElem = document.getElementById('setting-autostart');
        if (autostartElem) autostartElem.checked = !!settings.autostart;

        const trayElem = document.getElementById('setting-tray');
        if (trayElem) trayElem.checked = !!settings.minimize_to_tray;

        const appTrayElem = document.getElementById('setting-app-tray-icons');
        if (appTrayElem) appTrayElem.checked = false;

        const trayAppsMenuElem = document.getElementById('setting-tray-apps-menu');
        if (trayAppsMenuElem) trayAppsMenuElem.checked = false;

        const googleOauthFallbackElem = document.getElementById('setting-google-oauth-fallback');
        if (googleOauthFallbackElem) googleOauthFallbackElem.checked = false;

        document.getElementById('userdata-path').value = settings.current_userdata_path || '';
        document.getElementById('apps-path').textContent = settings.current_apps_path || '';
        document.getElementById('config-path').textContent = settings.current_config_path || '';
        document.getElementById('runtime-path').textContent = settings.current_runtime_path || '';
        document.getElementById('shared-storage-path').textContent = settings.current_shared_storage_path || '';

        // Автосохранение общих настроек при клике по чекбоксам
        if (autostartElem && !autostartElem.dataset.listener) {
            autostartElem.dataset.listener = 'true';
            autostartElem.addEventListener('change', saveGeneralSettings);
        }
        if (trayElem && !trayElem.dataset.listener) {
            trayElem.dataset.listener = 'true';
            trayElem.addEventListener('change', saveGeneralSettings);
        }
        if (appTrayElem && !appTrayElem.dataset.listener) {
            appTrayElem.dataset.listener = 'true';
            appTrayElem.addEventListener('change', saveGeneralSettings);
        }
        if (trayAppsMenuElem && !trayAppsMenuElem.dataset.listener) {
            trayAppsMenuElem.dataset.listener = 'true';
            trayAppsMenuElem.addEventListener('change', saveGeneralSettings);
        }
        if (googleOauthFallbackElem && !googleOauthFallbackElem.dataset.listener) {
            googleOauthFallbackElem.dataset.listener = 'true';
            googleOauthFallbackElem.addEventListener('change', saveGeneralSettings);
        }
    } catch (error) {
        console.error('Error loading engine settings:', error);
    }
}

async function saveGeneralSettings() {
    try {
        if (!managerAPI) return;
        const autostart = document.getElementById('setting-autostart')?.checked || false;
        const minimizeToTray = document.getElementById('setting-tray')?.checked || false;
        const appTrayIcons = document.getElementById('setting-app-tray-icons')?.checked ?? true;
        const trayAppsMenu = document.getElementById('setting-tray-apps-menu')?.checked || false;
        const googleOauthFallback = document.getElementById('setting-google-oauth-fallback')?.checked ?? true;

        const settings = {
            autostart: autostart,
            minimize_to_tray: minimizeToTray
        };

        await new Promise((resolve) => {
            managerAPI.updateEngineSettings(JSON.stringify(settings), function() {
                resolve();
            });
        });
        showNotification(translations[currentLang].settings_saved || 'Настройки сохранены', 'success');
    } catch (error) {
        console.error('Error saving general settings:', error);
    }
}

async function saveEngineSettings() {
    try {
        if (!managerAPI) {
            console.error('API not initialized');
            return;
        }

        const userdataPath = document.getElementById('userdata-path').value;
        const settingsJson = await new Promise(resolve => managerAPI.getEngineSettings(resolve));
        const currentPath = JSON.parse(settingsJson).current_userdata_path || '';

        if (userdataPath && userdataPath !== currentPath) {
            const options = await showUserdataChangeModal();
            if (!options) {
                await loadEngineSettings();
                return;
            }

            const resultJson = await new Promise(resolve => {
                managerAPI.changeUserdataPath(userdataPath, options.transferData, options.deleteOld, resolve);
            });
            const result = JSON.parse(resultJson || '{}');
            if (!result.success) {
                await loadEngineSettings();
                showNotification(result.error || translations[currentLang].error_saving, 'error');
                return;
            }
            showNotification(translations[currentLang].userdata_changed, 'success');
            setTimeout(() => loadEngineSettings(), 300);
            return;
        }

        const settings = {
            userdata_path: userdataPath
        };

        await new Promise((resolve, reject) => {
            managerAPI.updateEngineSettings(JSON.stringify(settings), function() {
                resolve();
            });
        });

        showNotification(translations[currentLang].settings_saved || 'Настройки сохранены', 'success');

        // Перезагружаем настройки чтобы увидеть обновленные пути
        setTimeout(() => loadEngineSettings(), 500);
    } catch (error) {
        console.error('Error saving engine settings:', error);
        showNotification(translations[currentLang].error_saving || 'Ошибка сохранения', 'error');
    }
}

async function selectUserdataPath() {
    try {
        if (!managerAPI) {
            console.error('API not initialized');
            return;
        }

        const currentPath = document.getElementById('userdata-path').value;

        await new Promise((resolve, reject) => {
            managerAPI.selectFolder(currentPath, function(newPath) {
                if (newPath) {
                    document.getElementById('userdata-path').value = newPath;
                }
                resolve();
            });
        });
    } catch (error) {
        console.error('Error selecting folder:', error);
    }
}

async function openFolder(folderType) {
    try {
        if (!managerAPI) {
            console.error('API not initialized');
            return;
        }

        managerAPI.openFolder(folderType);
    } catch (error) {
        console.error('Error opening folder:', error);
    }
}

// Обновление информации о хранилище
function updateStorageInfo() {
    if (!managerAPI || storageUpdateInProgress) return;
    storageUpdateInProgress = true;

    let pending = 0;
    const done = () => {
        pending -= 1;
        if (pending <= 0) storageUpdateInProgress = false;
    };

    if (managerAPI.getTotalCacheSize) {
        pending += 1;
        managerAPI.getTotalCacheSize(cacheSize => {
            const elem = document.getElementById('total-cache-size');
            if (elem) elem.textContent = cacheSize || '0 B';
            done();
        });
    }

    if (managerAPI.getTotalDataSize) {
        pending += 1;
        managerAPI.getTotalDataSize(dataSize => {
            const elem = document.getElementById('total-data-size');
            if (elem) elem.textContent = dataSize || '0 B';
            done();
        });
    }

    if (managerAPI.getAppStorageSizes) {
        pending += 1;
        managerAPI.getAppStorageSizes(sizesJson => {
            try {
                const sizes = JSON.parse(sizesJson);
                const container = document.getElementById('private-storage-list');
                if (container) {
                    container.innerHTML = sizes.length
                        ? sizes.map(item => `
                            <div class="storage-item">
                                <span class="storage-label">${item.name}</span>
                                <span class="storage-details">
                                    <span><span class="storage-detail-label">${translations[currentLang].private_cache_size}</span> ${item.cache_size || '0 B'}</span>
                                    <span><span class="storage-detail-label">${translations[currentLang].private_data_size}</span> ${item.data_size || '0 B'}</span>
                                </span>
                            </div>
                        `).join('')
                        : `<div class="storage-empty">${translations[currentLang].no_private_storage}</div>`;
                }
            } catch (error) {
                console.error('Error loading application storage sizes:', error);
            }
            done();
        });
    }

    if (pending === 0) storageUpdateInProgress = false;
}
