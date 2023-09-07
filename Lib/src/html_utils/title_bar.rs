/// The default minimize, maximize and close buttons
pub const DEFAULT_WINDOW_ACTIONS: &str = r#"
<div id="window-actions" class="drag-region">
<!--
    to change parameters dynamically using JS then use:
    const pathElement = document.querySelector(".window-icon-svg>path");
    pathElement.style.fill = "green";
-->
    <div class="window-icon titlebar-button" onclick="window.ipc.postMessage('minimize')">
        <svg class="window-icon-svg" viewBox="0 0 100 101" xmlns="http://www.w3.org/2000/svg">
            <path
                d="m21.325583 44.759563h57.350793c3.249547 0 5.865608 2.340284 5.865608 5.247274 0 2.906989-2.616061 5.247273-5.865608 5.247273h-57.350793c-3.249547 0-5.865608-2.340284-5.865608-5.247273 0-2.90699 2.616061-5.247274 5.865608-5.247274z"
                fill="" stroke-linecap="round" stroke-linejoin="round" stroke-width="10" />
        </svg>
    </div>
    <div class="window-icon titlebar-button" onclick="window.ipc.postMessage('maximize')">
        <svg class="window-icon-svg" viewBox="0 0 100 101" xmlns="http://www.w3.org/2000/svg">
            <path
                d="m50 22.033203a4.9999952 4.9999952 0 0 0 -5 5 4.9999952 4.9999952 0 0 0 5 5h5.744141 12.230468l-.002 12.230469v5.742187a4.9999952 4.9999952 0 0 0 5 5 4.9999952 4.9999952 0 0 0 5-5v-5.742187l.002-14.357422a5.0004952 5.0004952 0 0 0 0-.002c0-4.289877-3.584699-7.871094-7.873047-7.871094h-14.357421zm-22.972656 22.972656a4.9999952 4.9999952 0 0 0 -5 5v5.744141 14.357422c0 4.288347 3.582425 7.871094 7.871093 7.871094h14.359375 5.742188a4.9999952 4.9999952 0 0 0 5-5 4.9999952 4.9999952 0 0 0 -5-4.998047h-5.742188-12.230468v-12.230469-5.744141a4.9999952 4.9999952 0 0 0 -5-5z"
                fill="" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
    </div>
    <div class="window-icon titlebar-button" onclick="window.ipc.postMessage('close_window')"><svg class="window-icon-svg" viewBox="0 0 100 101" xmlns="http://www.w3.org/2000/svg">
            <path
                d="m23.241805 23.741809a5 5 0 0 0 0 7.071068l5.80601 5.806009 14.513643 14.513643c3.515097 3.515098 9.363131 3.513946 12.877077 0l14.513643-14.513643 5.80601-5.806009a5 5 0 0 0 0-7.071068 5 5 0 0 0 -7.071068 0l-5.80601 5.80601-13.881114 13.881113-13.881113-13.881113-5.80601-5.80601a5 5 0 0 0 -7.071068 0zm0 46.445314a5 5 0 0 0 0 7.071068 5 5 0 0 0 7.071068 0l5.80601-5.80601 13.881113-13.881113 13.881114 13.881113 5.804629 5.804629a5 5 0 0 0 7.071068 0 5 5 0 0 0 0-7.071068l-5.804629-5.804628-14.513643-14.513643c-3.513946-3.513947-9.362833-3.514244-12.877077 0l-14.513643 14.513643z"
                fill="" stroke-linecap="round" stroke-linejoin="round" />
        </svg></div>
</div>
"#;

/// The default styles for the default window actions as defined by [DEFAULT_WINDOW_ACTIONS] const
pub const DEFAULT_WINDOW_ACTIONS_STYLE: &str = r#"
#window-actions {
    display: flex;
    justify-content: end;
    -webkit-box-orient: horizontal;
    -webkit-box-direction: normal;
    flex-direction: row;
    -webkit-box-align: center;
    align-items: center;
}

.window-icon {
    padding: 10px;
}

.window-icon svg {
    width: 20px;
}

.window-icon-svg>path {
    fill: #FFFFFF;
    stroke-width: 3;
    stroke: none
}


.window-icon-svg>path:hover {
    fill: #00FFFF;
    stroke-width: 3px;
    stroke: none;
}
"#;

/// The default styles for the default window actions as defined by [DEFAULT_WINDOW_ACTIONS] const
pub const DEFAULT_WINDOW_ACTIONS_SCRIPT: &str = r#"
<script>
document.addEventListener('mousedown', (e) => {
    if (e.target.classList.contains('drag-region') && e.buttons === 1) {
        e.detail === 2
            ? window.ipc.postMessage('maximize')
            : window.ipc.postMessage('drag_window');
    }
})
document.addEventListener('touchstart', (e) => {
    if (e.target.classList.contains('drag-region')) {
        window.ipc.postMessage('drag_window');
    }
})
</script>
"#;
