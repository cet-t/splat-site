let f = function () {
  const modeEl = document.getElementById("mode");
  const whenEl = document.getElementById("when");
  const nRow = document.getElementById("n-row");
  const nEl = document.getElementById("n");
  const baseEl = document.getElementById("base");
  const out = document.getElementById("url-output");
  const copyBtn = document.getElementById("copy-btn");
  const copiedMsg = document.getElementById("copied-msg");

  function buildPath() {
    const mode = modeEl.value;
    const when = whenEl.value;
    if (when === "next") return `/${mode}/next`;
    return `/${mode}/now`;
  }

  function render() {
    nRow.style.display = whenEl.value === "next" ? "" : "none";
    const base = baseEl.value.replace(/\/+$/, "");
    const path = buildPath();
    const params = [];
    if (whenEl.value === "next") {
      const n = parseInt(nEl.value, 10);
      if (!isNaN(n) && n !== 1) params.push(`n=${n}`);
    }
    params.push("t={timestamp}");
    out.textContent = `${base}${path}?${params.join("&")}`;
  }

  async function copy() {
    const base = baseEl.value.replace(/\/+$/, "");
    const path = buildPath();
    const params = [];
    if (whenEl.value === "next") {
      const n = parseInt(nEl.value, 10);
      if (!isNaN(n) && n !== 1) params.push(`n=${n}`);
    }
    const t = Math.floor(Date.now() / 1000);
    params.push(`t=${t}`);
    const url = `${base}${path}?${params.join("&")}`;
    try {
      await navigator.clipboard.writeText(url);
    } catch (e) {
      const ta = document.createElement("textarea");
      ta.value = url;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
    }
    out.textContent = url;
    copiedMsg.classList.add("show");
    setTimeout(() => copiedMsg.classList.remove("show"), 1500);
  }

  [modeEl, whenEl, nEl, baseEl].forEach((el) =>
    el.addEventListener("input", render),
  );
  copyBtn.addEventListener("click", copy);
  render();
};

f();
