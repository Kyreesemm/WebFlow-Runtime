let managerAPI = null;
window.currentImportedCookies = [];
let defaultIsolatedStorage = true;
let managerIsGnome = false;
let savedManagerFileLogging = false;
let updateAvailable = false;
let startupUpdateCheckStarted = false;
let lastUpdateState = null;

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
        template_category: 'Шаблоны',
        add_useragent: 'Добавить User-Agent',
        name: 'Название',
        useragent_string: 'User-Agent строка',
        add: 'Добавить',
        general_settings: 'Общие параметры',
        update_settings_title: 'Параметры обновлений',
        check_updates_on_startup: 'Проверять обновления при запуске менеджера',
        check_updates_on_startup_desc: 'Проверять наличие новой версии при каждом запуске WebFlow Runtime Manager',
        check_updates_on_startup_warning: 'При включении этой функции частые перезапуски менеджера могут привести к блокировке проверки обновлений на сутки. Включайте эту настройку с осторожностью.',
        enable: 'Включить',
        updates_not_checked: 'Проверка не выполнялась',
        updates_not_checked_desc: 'Запустите проверку вручную или включите её при запуске менеджера',
        project_tab: 'О проекте',
        project_branding_desc: 'Лёгкая кроссплатформенная среда для превращения веб-сайтов в приложения',
        project_version_label: 'Версия',
        project_developer: 'Разработчик',
        project_license: 'Лицензия',
        project_version_title: 'Версия проекта',
        project_updates_title: 'Обновления',
        updates_checking: 'Проверка обновлений...',
        updates_checking_desc: 'Получение информации о релизах GitHub',
        updates_current: 'Установлена актуальная версия',
        updates_current_desc: 'Новых совместимых релизов не найдено',
        updates_available: 'Доступно обновление до {version}',
        updates_available_desc: 'Доступен совместимый релиз для этой системы',
        updates_error: 'Не удалось проверить обновления',
        updates_downloading: 'Скачивание обновления...',
        updates_verifying: 'Проверка целостности...',
        updates_restarting: 'Перезапуск WebFlow Runtime...',
        updates_error_desc: 'Повторите попытку позже',
        update_button: 'Обновить',
        check_updates_button: 'Проверить обновления',
        update_found_notification: 'Доступно обновление до {version}',
        updates_current_notification: 'Установлена актуальная версия',
        runtime_components_title: 'Компоненты WebView',
        component_only_linux: 'Доступно только в Linux',
        component_only_windows: 'Доступно только в Windows',
        component_version_unavailable: 'Версия недоступна',
        project_links_title: 'Ссылки проекта',
        github_link: 'GitHub',
        documentation_link: 'Документация',
        issues_link: 'Issues',
        version_channel_dev: 'dev-сборка',
        developer_settings: 'Для разработчиков',
        manager_file_logging: 'Сохранять логи менеджера в файл',
        manager_file_logging_desc: 'Записывать подробные логи каждого запуска менеджера в папку logs',
        app_file_logging: 'Сохранять логи приложений в файл',
        app_file_logging_desc: 'Запускать приложения из менеджера с подробным логированием в папку logs',
        manager_logging_restart: 'Настройка сохранена. Менеджер перезапускается',
        autostart: 'Автозапуск менеджера',
        autostart_desc: 'Запускать менеджер при старте системы',
        start_minimized: 'Запускать свёрнутым',
        start_minimized_desc: 'Запускать менеджер в системном трее при старте системы',
        manager_minimize_tray: 'Минимизация менеджера в трей',
        manager_minimize_tray_desc: 'Сворачивать менеджер в системный трей при закрытии окна',
        app_tray_icons: 'Иконки приложений в трее',
        app_tray_icons_desc: 'Отображать отдельную иконку в системном трее для каждого запущенного приложения',
        app_minimize_tray: 'Минимизация приложений в трей',
        app_minimize_tray_desc: 'Сворачивать отдельные приложения в системный трей при закрытии окна',
        tray_apps_menu: 'Меню приложений в трее менеджера',
        tray_apps_menu_desc: 'Показывать список приложений в контекстном меню иконки трея менеджера',
        storage_mode: 'Режим хранилища',
        isolated_storage_desc: 'Каждое приложение использует отдельное хранилище куки и данных',
        data_management: 'Управление данными',
        clear_cache: 'Очистить кэш всех приложений',
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
        template_category: 'Templates',
        add_useragent: 'Add User-Agent',
        name: 'Name',
        useragent_string: 'User-Agent String',
        add: 'Add',
        general_settings: 'General Settings',
        update_settings_title: 'Update Settings',
        check_updates_on_startup: 'Check for updates when the manager starts',
        check_updates_on_startup_desc: 'Check for a new version every time WebFlow Runtime Manager starts',
        check_updates_on_startup_warning: 'When enabled, frequent manager restarts may cause update checks to be blocked for 24 hours. Enable this setting with care.',
        enable: 'Enable',
        updates_not_checked: 'Updates have not been checked',
        updates_not_checked_desc: 'Start a manual check or enable checking when the manager starts',
        project_tab: 'About',
        project_branding_desc: 'A lightweight cross-platform environment for turning websites into applications',
        project_version_label: 'Version',
        project_developer: 'Developer',
        project_license: 'License',
        project_version_title: 'Project Version',
        project_updates_title: 'Updates',
        updates_checking: 'Checking for updates...',
        updates_checking_desc: 'Fetching release information from GitHub',
        updates_current: 'The latest version is installed',
        updates_current_desc: 'No newer compatible release was found',
        updates_available: 'Update available: {version}',
        updates_available_desc: 'A compatible release is available for this system',
        updates_error: 'Update check failed',
        updates_downloading: 'Downloading update...',
        updates_verifying: 'Verifying update integrity...',
        updates_restarting: 'Restarting WebFlow Runtime...',
        updates_error_desc: 'Please try again later',
        update_button: 'Update',
        check_updates_button: 'Check for Updates',
        update_found_notification: 'Update available: {version}',
        updates_current_notification: 'The latest version is already installed',
        runtime_components_title: 'WebView Components',
        component_only_linux: 'Available only on Linux',
        component_only_windows: 'Available only on Windows',
        component_version_unavailable: 'Version unavailable',
        project_links_title: 'Project Links',
        github_link: 'GitHub',
        documentation_link: 'Documentation',
        issues_link: 'Issues',
        version_channel_dev: 'dev build',
        developer_settings: 'For Developers',
        manager_file_logging: 'Save Manager Logs to File',
        manager_file_logging_desc: 'Write detailed logs for every manager launch to the logs folder',
        app_file_logging: 'Save Application Logs to File',
        app_file_logging_desc: 'Launch applications from the Manager with detailed logging to the logs folder',
        manager_logging_restart: 'Setting saved. The Manager is restarting',
        autostart: 'Manager Autostart',
        autostart_desc: 'Start manager on system startup',
        start_minimized: 'Start Minimized',
        start_minimized_desc: 'Start the manager in the system tray with the system',
        manager_minimize_tray: 'Minimize Manager to Tray',
        manager_minimize_tray_desc: 'Minimize the Manager to the system tray when its window is closed',
        app_tray_icons: 'Application Tray Icons',
        app_tray_icons_desc: 'Show a separate system tray icon for each running application',
        app_minimize_tray: 'Minimize Applications to Tray',
        app_minimize_tray_desc: 'Minimize individual applications to the system tray when their windows are closed',
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
    updateProjectComponentLabels();
    refreshUpdateStatusLocalization();

    // Обновить tooltips для вкладок
    document.querySelectorAll('.tab-icon').forEach(tab => {
        const tabName = tab.getAttribute('data-tab');
        const tooltips = {
            apps: currentLang === 'ru' ? 'Приложения' : 'Applications',
            settings: currentLang === 'ru' ? 'Общие настройки' : 'General Settings',
            storage: currentLang === 'ru' ? 'Хранилище' : 'Storage',
            useragents: currentLang === 'ru' ? 'User-Agent\'ы' : 'User-Agents',
            engine: currentLang === 'ru' ? 'Настройки движка' : 'Engine Settings',
            project: translations[currentLang].project_tab
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

function updateProjectComponentLabels() {
    const webkit = document.getElementById('webkitgtk-version');
    const webview2 = document.getElementById('webview2-version');
    if (!webkit || !webview2) return;

    if (navigator.userAgent.includes('Edg/')) {
        webkit.textContent = translations[currentLang].component_only_linux;
        webkit.dataset.platformOnly = 'linux';
    } else {
        webview2.textContent = translations[currentLang].component_only_windows;
        webview2.dataset.platformOnly = 'windows';
    }
}

function loadProjectInfo() {
    if (!managerAPI || !managerAPI.getRuntimeInfo) return;
    managerAPI.getRuntimeInfo(function(result) {
        try {
            const info = JSON.parse(result);
            const versionElem = document.getElementById('project-version');
            const channelElem = document.getElementById('project-version-channel');
            if (versionElem) versionElem.textContent = info.version ? `v${info.version}` : 'v—';
            if (channelElem) {
                const channel = (info.version || '').includes('dev')
                    ? translations[currentLang].version_channel_dev
                    : '';
                channelElem.textContent = channel ? `(${channel})` : '';
            }

            const webkitElem = document.getElementById('webkitgtk-version');
            const webview2Elem = document.getElementById('webview2-version');
            const isWindowsWebView = navigator.userAgent.includes('Edg/');
            if (isWindowsWebView) {
                const match = navigator.userAgent.match(/Edg\/([\d.]+)/);
                if (webkitElem) {
                    webkitElem.textContent = translations[currentLang].component_only_linux;
                    webkitElem.dataset.platformOnly = 'linux';
                }
                if (webview2Elem) {
                    webview2Elem.textContent = match ? match[1] : translations[currentLang].component_version_unavailable;
                }
            } else {
                if (webkitElem) webkitElem.textContent = info.webkitgtk_version || translations[currentLang].component_version_unavailable;
                if (webview2Elem) {
                    webview2Elem.textContent = translations[currentLang].component_only_windows;
                    webview2Elem.dataset.platformOnly = 'windows';
                }
            }
        } catch (error) {
            console.error('Error loading project information:', error);
        }
    });
}

function setUpdateStatus(status, description, canUpdate = false, action = 'check') {
    const statusElement = document.getElementById('project-update-status');
    const descriptionElement = document.getElementById('project-update-description');
    const button = document.getElementById('project-update-button');
    if (statusElement) statusElement.textContent = status;
    if (descriptionElement) descriptionElement.textContent = description;
    if (button) {
        button.disabled = !canUpdate;
        button.className = action === 'update' ? 'btn' : 'btn btn-secondary';
        button.innerHTML = `<span class="material-symbols-rounded">${action === 'update' ? 'download' : 'refresh'}</span><span>${action === 'update' ? translations[currentLang].update_button : translations[currentLang].check_updates_button}</span>`;
    }
}

function refreshUpdateStatusLocalization() {
    if (lastUpdateState) {
        applyUpdateState(lastUpdateState, false);
        return;
    }

    const checkUpdatesOnStartupElem = document.getElementById('setting-check-updates-on-startup');
    if (checkUpdatesOnStartupElem?.checked) {
        setUpdateStatus(
            translations[currentLang].updates_checking,
            translations[currentLang].updates_checking_desc
        );
    } else {
        setUpdateStatus(
            translations[currentLang].updates_not_checked,
            translations[currentLang].updates_not_checked_desc,
            true,
            'check'
        );
    }
}

function checkForUpdates(force = false, notify = false) {
    if (!managerAPI?.checkForUpdates) return;
    let notificationSent = false;
    setUpdateStatus(translations[currentLang].updates_checking, translations[currentLang].updates_checking_desc);
    managerAPI.checkForUpdates(force, function(result) {
        applyUpdateState(result, false);
        if (managerAPI.getUpdateState) {
            let attempts = 0;
            const poll = setInterval(() => {
                managerAPI.getUpdateState(state => {
                    let parsed;
                    try { parsed = JSON.parse(state); } catch (_) { parsed = null; }
                    const shouldNotify = notify && !notificationSent && parsed && parsed.status !== 'checking';
                    applyUpdateState(state, shouldNotify);
                    if (shouldNotify) notificationSent = true;
                    attempts += 1;
                    if (!parsed || parsed.status !== 'checking' || attempts >= 120) clearInterval(poll);
                });
            }, 500);
        }
    });
}

function applyUpdateState(result, notify = false) {
    try {
            const state = typeof result === 'string' ? JSON.parse(result) : result;
            lastUpdateState = state;
            const update = state.result || state;
            if (state.status === 'checking') {
                setUpdateStatus(translations[currentLang].updates_checking, translations[currentLang].updates_checking_desc);
                return;
            }
            if (update.status === 'update_available') {
                updateAvailable = true;
                setUpdateStatus(
                    translations[currentLang].updates_available.replace('{version}', `v${update.latest_version}`),
                    translations[currentLang].updates_available_desc,
                    true,
                    'update'
                );
                if (notify) showNotification(translations[currentLang].update_found_notification.replace('{version}', `v${update.latest_version}`), 'success');
            } else if (update.status === 'up_to_date') {
                updateAvailable = false;
                setUpdateStatus(translations[currentLang].updates_current, translations[currentLang].updates_current_desc, true, 'check');
                if (notify) showNotification(translations[currentLang].updates_current_notification, 'success');
            } else {
                updateAvailable = false;
                setUpdateStatus(translations[currentLang].updates_error, state.message || update.error || translations[currentLang].updates_error_desc, true, 'check');
            }
            if (['downloading', 'verifying', 'verified', 'restarting'].includes(state.status)) {
                const progress = document.getElementById('project-update-progress');
                const bar = document.getElementById('project-update-progress-bar');
                if (progress) progress.hidden = false;
                if (bar) bar.style.width = `${state.percent || 0}%`;
                setUpdateStatus(
                    state.status === 'verifying' || state.status === 'verified' ? translations[currentLang].updates_verifying :
                        state.status === 'restarting' ? translations[currentLang].updates_restarting : translations[currentLang].updates_downloading,
                    state.message || '',
                    false,
                    'update'
                );
            } else if (state.status === 'error') {
                updateAvailable = false;
                setUpdateStatus(translations[currentLang].updates_error, state.message || translations[currentLang].updates_error_desc, true, 'check');
            }
        } catch (_) {
            setUpdateStatus(translations[currentLang].updates_error, translations[currentLang].updates_error_desc);
        }
}

function startUpdate() {
    const button = document.getElementById('project-update-button');
    const progress = document.getElementById('project-update-progress');
    if (button) button.disabled = true;
    if (progress) progress.hidden = false;
    setUpdateStatus(translations[currentLang].updates_downloading, translations[currentLang].updates_available_desc, false, 'update');
    managerAPI?.startUpdate?.(() => {});
    let attempts = 0;
    const poll = setInterval(() => {
        managerAPI?.getUpdateState?.(state => {
            applyUpdateState(state);
            attempts += 1;
            let parsed;
            try { parsed = JSON.parse(state); } catch (_) { parsed = null; }
            if (!parsed || ['error', 'restarting'].includes(parsed.status) || attempts >= 360) clearInterval(poll);
        });
    }, 500);
}

function handleUpdateButton() {
    if (updateAvailable) startUpdate();
    else checkForUpdates(true, true);
}

document.getElementById('project-update-button')?.addEventListener('click', handleUpdateButton);

window.__webflowUpdateProgress = function(update) {
    const bar = document.getElementById('project-update-progress-bar');
    const statusElement = document.getElementById('project-update-status');
    const descriptionElement = document.getElementById('project-update-description');
    const progress = document.getElementById('project-update-progress');
    if (progress) progress.hidden = false;
    if (bar) bar.style.width = `${Math.max(0, Math.min(100, update.percent || 0))}%`;
    if (update.phase === 'verifying' || update.phase === 'verified') {
        if (statusElement) statusElement.textContent = translations[currentLang].updates_verifying;
    } else if (update.phase === 'restarting') {
        if (statusElement) statusElement.textContent = translations[currentLang].updates_restarting;
    } else if (update.phase === 'error') {
        if (statusElement) statusElement.textContent = translations[currentLang].updates_error;
        if (descriptionElement) descriptionElement.textContent = update.message || translations[currentLang].updates_error_desc;
        if (progress) progress.hidden = true;
        const button = document.getElementById('project-update-button');
        if (button) button.disabled = false;
    }
    if (update.phase !== 'error' && descriptionElement && update.message) descriptionElement.textContent = update.message;
};

function openProjectLink(url) {
    if (managerAPI?.openProjectLink) managerAPI.openProjectLink(url);
}

function setupBrandingIconTilt() {
    const icon = document.querySelector('.project-branding-icon');
    if (!icon || icon.dataset.tiltReady) return;
    icon.dataset.tiltReady = 'true';

    icon.addEventListener('mousemove', event => {
        const bounds = icon.getBoundingClientRect();
        const offsetX = (event.clientX - bounds.left) / bounds.width - 0.5;
        const offsetY = (event.clientY - bounds.top) / bounds.height - 0.5;
        icon.style.setProperty('--tilt-x', `${(-offsetY * 8).toFixed(2)}deg`);
        icon.style.setProperty('--tilt-y', `${(offsetX * 8).toFixed(2)}deg`);
    });
    icon.addEventListener('mouseleave', () => {
        icon.style.setProperty('--tilt-x', '0deg');
        icon.style.setProperty('--tilt-y', '0deg');
    });
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
    loadProjectInfo();
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
    loadEngineSettings();
    loadProjectInfo();
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
    setupBrandingIconTilt();
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
    modal.classList.remove('closing');
    modal.classList.add('active');

    return new Promise(resolve => {
        confirmResolver = resolve;
    });
}

function closeModalWithAnimation(modal, onClosed = null) {
    if (!modal || !modal.classList.contains('active')) return;

    modal.classList.add('closing');
    setTimeout(() => {
        if (!modal.classList.contains('closing')) return;
        modal.classList.remove('active', 'closing');
        if (onClosed) onClosed();
    }, 300);
}

function closeConfirmModal(accepted) {
    const modal = document.getElementById('confirm-modal');
    closeModalWithAnimation(modal);

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
    modal.classList.remove('closing');
    modal.classList.add('active');
    return new Promise(resolve => { userdataChangeResolver = resolve; });
}

function closeUserdataChangeModal(accepted) {
    closeModalWithAnimation(document.getElementById('userdata-change-modal'));
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
    document.getElementById('isolated-storage').checked = defaultIsolatedStorage;
    document.getElementById('icon-preview').style.display = 'none';
    window.currentImportedCookies = [];
    updateFileInputLabel();
    document.getElementById('modal').classList.remove('closing');
    document.getElementById('modal').classList.add('active');
}

// Закрыть модальное окно
function closeModal() {
    const modal = document.getElementById('modal');
    closeModalWithAnimation(modal);
}

// Показать модальное окно шаблонов
function showTemplatesModal() {
    if (!managerAPI) return;

    managerAPI.listTemplates(function(templatesJson) {
        const templates = JSON.parse(templatesJson);
        const container = document.getElementById('templates-list');

        const grouped = templates.reduce((groups, template) => {
            const category = typeof template.category === 'object'
                ? (template.category[currentLang] || template.category.en || template.category.ru)
                : (template.category || translations[currentLang].template_category);
            (groups[category] ||= []).push(template);
            return groups;
        }, {});

        container.innerHTML = Object.entries(grouped).map(([category, categoryTemplates]) => `
            <section class="template-category">
                <h3>${category}</h3>
                <div class="template-grid">
                    ${categoryTemplates.map(template => {
            // Поддержка как старого формата (строка), так и нового (объект с переводами)
            const name = typeof template.name === 'object'
                ? (template.name[currentLang] || template.name.en || template.name.ru)
                : template.name;

            const description = typeof template.description === 'object'
                ? (template.description[currentLang] || template.description.en || template.description.ru)
                : template.description;

            // Иконка шаблона
            const iconHtml = template.icon_data
                ? `<img src="${template.icon_data}" alt="${name}">`
                : `<span>${name.charAt(0).toUpperCase()}</span>`;

            return `
                <div class="app-card template-card" onclick="createFromTemplate('${template.id}')">
                    <div class="app-card-header">
                        <div class="app-icon">${iconHtml}</div>
                        <div class="app-info">
                            <h3>${name}</h3>
                            <p>${description}</p>
                        </div>
                    </div>
                </div>
            `;
                    }).join('')}
                </div>
            </section>
        `).join('');

        const modal = document.getElementById('templates-modal');
        modal.classList.remove('closing');
        modal.classList.add('active');
    });
}

function closeTemplatesModal() {
    const modal = document.getElementById('templates-modal');
    closeModalWithAnimation(modal);
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
    const modal = document.getElementById('ua-modal');
    modal.classList.remove('closing');
    modal.classList.add('active');
}

function closeUAModal() {
    const modal = document.getElementById('ua-modal');
    closeModalWithAnimation(modal);
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
                closeModalWithAnimation(this);
            }
        }
    });
});

// Модальное окно настроек приложения
let currentAppId = null;

function openAppSettings(appId) {
    currentAppId = appId;
    const modal = document.getElementById('app-settings-modal');
    modal.classList.remove('closing');
    modal.classList.add('active');
}

function closeAppSettingsModal() {
    const modal = document.getElementById('app-settings-modal');
    closeModalWithAnimation(modal);
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
    const modal = document.getElementById('import-cookies-modal');
    modal.classList.remove('closing');
    modal.classList.add('active');
}

function closeImportCookiesModal() {
    const modal = document.getElementById('import-cookies-modal');
    closeModalWithAnimation(modal);
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

function updateStartMinimizedAvailability() {
    const autostartElem = document.getElementById('setting-autostart');
    const trayElem = document.getElementById('setting-tray');
    const startMinimizedElem = document.getElementById('setting-start-minimized');
    if (!autostartElem || !trayElem || !startMinimizedElem) return;

    const available = autostartElem.checked && trayElem.checked && !managerIsGnome;
    startMinimizedElem.disabled = !available;
    startMinimizedElem.closest('.settings-row')?.classList.toggle('feature-unavailable', !available);
    if (!available) startMinimizedElem.checked = false;
}

function updateAppMinimizeTrayAvailability() {
    const appTrayIconsElem = document.getElementById('setting-app-tray-icons');
    const appMinimizeElem = document.getElementById('setting-app-minimize-tray');
    if (!appTrayIconsElem || !appMinimizeElem) return;

    const available = appTrayIconsElem.checked && !appTrayIconsElem.disabled && !managerIsGnome;
    appMinimizeElem.disabled = !available;
    appMinimizeElem.closest('.settings-row')?.classList.toggle('feature-unavailable', !available);
    if (!available) appMinimizeElem.checked = false;
}

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
        const isolatedStorageElem = document.getElementById('setting-isolated-storage');
        if (isolatedStorageElem) {
            isolatedStorageElem.checked = settings.isolated_storage !== false;
            defaultIsolatedStorage = isolatedStorageElem.checked;
        }

        managerIsGnome = settings.is_gnome === true;

        const autostartElem = document.getElementById('setting-autostart');
        if (autostartElem) autostartElem.checked = !!settings.autostart;

        const trayElem = document.getElementById('setting-tray');
        if (trayElem) {
            trayElem.checked = !managerIsGnome && !!settings.minimize_to_tray;
            trayElem.disabled = managerIsGnome;
            trayElem.closest('.settings-row')?.classList.toggle('feature-unavailable', managerIsGnome);
        }

        const startMinimizedElem = document.getElementById('setting-start-minimized');
        if (startMinimizedElem) startMinimizedElem.checked = !!settings.start_minimized;
        updateStartMinimizedAvailability();

        const appTrayElem = document.getElementById('setting-app-tray-icons');
        if (appTrayElem) appTrayElem.checked = !!settings.app_tray_icons;

        const appMinimizeElem = document.getElementById('setting-app-minimize-tray');
        if (appMinimizeElem) appMinimizeElem.checked = !!settings.app_minimize_to_tray;
        updateAppMinimizeTrayAvailability();

        const trayAppsMenuElem = document.getElementById('setting-tray-apps-menu');
        if (trayAppsMenuElem) trayAppsMenuElem.checked = false;

        const googleOauthFallbackElem = document.getElementById('setting-google-oauth-fallback');
        if (googleOauthFallbackElem) googleOauthFallbackElem.checked = false;

        const managerFileLoggingElem = document.getElementById('setting-manager-file-logging');
        if (managerFileLoggingElem) {
            managerFileLoggingElem.checked = !!settings.manager_log_to_file;
            savedManagerFileLogging = managerFileLoggingElem.checked;
        }

        const appFileLoggingElem = document.getElementById('setting-app-file-logging');
        if (appFileLoggingElem) appFileLoggingElem.checked = !!settings.app_log_to_file;

        const checkUpdatesOnStartupElem = document.getElementById('setting-check-updates-on-startup');
        if (checkUpdatesOnStartupElem) {
            checkUpdatesOnStartupElem.checked = !!settings.check_updates_on_startup;
            if (checkUpdatesOnStartupElem.checked && !startupUpdateCheckStarted) {
                startupUpdateCheckStarted = true;
                checkForUpdates(false, true);
            } else if (!checkUpdatesOnStartupElem.checked) {
                if (lastUpdateState) {
                    refreshUpdateStatusLocalization();
                } else {
                    setUpdateStatus(
                        translations[currentLang].updates_not_checked,
                        translations[currentLang].updates_not_checked_desc,
                        true,
                        'check'
                    );
                }
            }
        }

        document.getElementById('userdata-path').value = settings.current_userdata_path || '';
        document.getElementById('apps-path').textContent = settings.current_apps_path || '';
        document.getElementById('config-path').textContent = settings.current_config_path || '';
        document.getElementById('runtime-path').textContent = settings.current_runtime_path || '';
        document.getElementById('shared-storage-path').textContent = settings.current_shared_storage_path || '';

        // Автосохранение общих настроек при клике по чекбоксам
        if (autostartElem && !autostartElem.dataset.listener) {
            autostartElem.dataset.listener = 'true';
            autostartElem.addEventListener('change', () => {
                updateStartMinimizedAvailability();
                saveGeneralSettings();
            });
        }
        if (trayElem && !trayElem.dataset.listener) {
            trayElem.dataset.listener = 'true';
            trayElem.addEventListener('change', () => {
                updateStartMinimizedAvailability();
                saveGeneralSettings();
            });
        }
        if (startMinimizedElem && !startMinimizedElem.dataset.listener) {
            startMinimizedElem.dataset.listener = 'true';
            startMinimizedElem.addEventListener('change', saveGeneralSettings);
        }
        if (appTrayElem && !appTrayElem.dataset.listener) {
            appTrayElem.dataset.listener = 'true';
            appTrayElem.addEventListener('change', () => {
                updateAppMinimizeTrayAvailability();
                saveGeneralSettings();
            });
        }
        if (appMinimizeElem && !appMinimizeElem.dataset.listener) {
            appMinimizeElem.dataset.listener = 'true';
            appMinimizeElem.addEventListener('change', saveGeneralSettings);
        }
        if (trayAppsMenuElem && !trayAppsMenuElem.dataset.listener) {
            trayAppsMenuElem.dataset.listener = 'true';
            trayAppsMenuElem.addEventListener('change', saveGeneralSettings);
        }
        if (googleOauthFallbackElem && !googleOauthFallbackElem.dataset.listener) {
            googleOauthFallbackElem.dataset.listener = 'true';
            googleOauthFallbackElem.addEventListener('change', saveGeneralSettings);
        }
        if (isolatedStorageElem && !isolatedStorageElem.dataset.listener) {
            isolatedStorageElem.dataset.listener = 'true';
            isolatedStorageElem.addEventListener('change', saveGeneralSettings);
        }
        if (managerFileLoggingElem && !managerFileLoggingElem.dataset.listener) {
            managerFileLoggingElem.dataset.listener = 'true';
            managerFileLoggingElem.addEventListener('change', saveGeneralSettings);
        }
        if (appFileLoggingElem && !appFileLoggingElem.dataset.listener) {
            appFileLoggingElem.dataset.listener = 'true';
            appFileLoggingElem.addEventListener('change', saveGeneralSettings);
        }
        if (checkUpdatesOnStartupElem && !checkUpdatesOnStartupElem.dataset.listener) {
            checkUpdatesOnStartupElem.dataset.listener = 'true';
            checkUpdatesOnStartupElem.addEventListener('change', async () => {
                if (checkUpdatesOnStartupElem.checked) {
                    const confirmed = await showConfirm(
                        translations[currentLang].check_updates_on_startup_warning,
                        translations[currentLang].enable
                    );
                    if (!confirmed) {
                        checkUpdatesOnStartupElem.checked = false;
                        return;
                    }
                }
                saveGeneralSettings();
            });
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
        const isolatedStorage = document.getElementById('setting-isolated-storage')?.checked ?? true;
        const startMinimized = document.getElementById('setting-start-minimized')?.checked ?? false;
        const appMinimizeToTray = document.getElementById('setting-app-minimize-tray')?.checked || false;
        const managerFileLogging = document.getElementById('setting-manager-file-logging')?.checked ?? false;
        const appFileLogging = document.getElementById('setting-app-file-logging')?.checked ?? false;
        const checkUpdatesOnStartup = document.getElementById('setting-check-updates-on-startup')?.checked ?? false;
        const managerLoggingChanged = managerFileLogging !== savedManagerFileLogging;
        defaultIsolatedStorage = isolatedStorage;

        const settings = {
            autostart,
            minimize_to_tray: minimizeToTray,
            app_tray_icons: appTrayIcons,
            app_minimize_to_tray: appMinimizeToTray,
            start_minimized: startMinimized,
            isolated_storage: isolatedStorage,
            manager_log_to_file: managerFileLogging,
            app_log_to_file: appFileLogging,
            check_updates_on_startup: checkUpdatesOnStartup
        };

        await new Promise((resolve) => {
            managerAPI.updateEngineSettings(JSON.stringify(settings), function() {
                resolve();
            });
        });
        savedManagerFileLogging = managerFileLogging;
        if (managerLoggingChanged) {
            showNotification(translations[currentLang].manager_logging_restart, 'success');
            setTimeout(() => managerAPI.restartManager(managerFileLogging), 80);
            return;
        }
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
