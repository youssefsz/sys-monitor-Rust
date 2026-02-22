/* ────────────────────────────────────────────────────────────────
   sys-monitor landing page - main.js
   ──────────────────────────────────────────────────────────────── */

(function () {
    "use strict";

    // ── Mobile nav toggle ──────────────────────────────────────────

    var navToggle = document.getElementById("nav-toggle");
    var navLinks = document.getElementById("nav-links");

    if (navToggle && navLinks) {
        navToggle.addEventListener("click", function () {
            navLinks.classList.toggle("open");
        });
    }

    // ── Install tab switching ──────────────────────────────────────

    var tabs = document.querySelectorAll(".install-tab");
    var cmdBlocks = document.querySelectorAll(".install-cmd");

    tabs.forEach(function (tab) {
        tab.addEventListener("click", function () {
            var targetId = tab.getAttribute("data-target");

            // Deactivate all tabs
            tabs.forEach(function (t) {
                t.classList.remove("active");
            });

            // Hide all command blocks
            cmdBlocks.forEach(function (block) {
                block.classList.add("install-cmd-hidden");
            });

            // Activate selected
            tab.classList.add("active");
            var target = document.getElementById(targetId);
            if (target) {
                target.classList.remove("install-cmd-hidden");
            }
        });
    });

    // ── Copy to clipboard ─────────────────────────────────────────

    var copyButtons = document.querySelectorAll(".copy-btn");

    copyButtons.forEach(function (btn) {
        btn.addEventListener("click", function () {
            var codeId = btn.getAttribute("data-clipboard");
            var codeEl = document.getElementById(codeId);

            if (!codeEl) return;

            var text = codeEl.textContent.trim();

            navigator.clipboard.writeText(text).then(function () {
                btn.textContent = "Copied";
                btn.classList.add("copied");
                setTimeout(function () {
                    btn.textContent = "Copy";
                    btn.classList.remove("copied");
                }, 2000);
            }).catch(function () {
                // Fallback for older browsers
                var textarea = document.createElement("textarea");
                textarea.value = text;
                textarea.style.position = "fixed";
                textarea.style.left = "-9999px";
                document.body.appendChild(textarea);
                textarea.select();
                try {
                    document.execCommand("copy");
                    btn.textContent = "Copied";
                    btn.classList.add("copied");
                    setTimeout(function () {
                        btn.textContent = "Copy";
                        btn.classList.remove("copied");
                    }, 2000);
                } catch (e) {
                    // silently fail
                }
                document.body.removeChild(textarea);
            });
        });
    });

    // ── Docs code block copy buttons ─────────────────────────────

    var codeCopyButtons = document.querySelectorAll(".code-copy-btn");

    codeCopyButtons.forEach(function (btn) {
        btn.addEventListener("click", function () {
            var pre = btn.parentElement.querySelector("pre");
            if (!pre) return;

            var text = pre.textContent.trim();

            navigator.clipboard.writeText(text).then(function () {
                btn.textContent = "Copied";
                btn.classList.add("copied");
                setTimeout(function () {
                    btn.textContent = "Copy";
                    btn.classList.remove("copied");
                }, 2000);
            }).catch(function () {
                var textarea = document.createElement("textarea");
                textarea.value = text;
                textarea.style.position = "fixed";
                textarea.style.left = "-9999px";
                document.body.appendChild(textarea);
                textarea.select();
                try {
                    document.execCommand("copy");
                    btn.textContent = "Copied";
                    btn.classList.add("copied");
                    setTimeout(function () {
                        btn.textContent = "Copy";
                        btn.classList.remove("copied");
                    }, 2000);
                } catch (e) {
                    // silently fail
                }
                document.body.removeChild(textarea);
            });
        });
    });

    // ── Docs sidebar active link ──────────────────────────────────

    var docLinks = document.querySelectorAll(".docs-nav a");
    if (docLinks.length > 0) {
        var sections = [];
        docLinks.forEach(function (link) {
            var href = link.getAttribute("href");
            if (href && href.startsWith("#")) {
                var section = document.getElementById(href.substring(1));
                if (section) {
                    sections.push({ link: link, section: section });
                }
            }
        });

        if (sections.length > 0) {
            window.addEventListener("scroll", function () {
                var scrollPos = window.scrollY + 120;

                var current = sections[0];
                for (var i = 0; i < sections.length; i++) {
                    if (sections[i].section.offsetTop <= scrollPos) {
                        current = sections[i];
                    }
                }

                docLinks.forEach(function (link) {
                    link.classList.remove("active");
                });

                if (current) {
                    current.link.classList.add("active");
                }
            });
        }
    }
})();
