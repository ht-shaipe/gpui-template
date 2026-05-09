async function init() {
  const loadingEl = document.getElementById('loading');

  try {
    const wasm = await import('./wasm/gpui_template.js');
    await wasm.default();

    // init_web is synchronous — do NOT await it.
    // GPUI schedules all work via spawn_local internally.
    wasm.init_web();

    if (loadingEl) {
      loadingEl.remove();
    }
  } catch (error) {
    console.error('Failed to initialize:', error);
    if (loadingEl) {
      loadingEl.innerHTML = `
        <div class="error">
          <h2>Failed to load the application</h2>
          <p>${error.message || error}</p>
          <p style="margin-top: 10px; font-size: 14px;">
            Run <code>./scripts/build-wasm.sh</code> from the repo root, then refresh.
          </p>
        </div>
      `;
    }
  }
}

init();
