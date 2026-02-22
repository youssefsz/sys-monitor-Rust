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
            var isOpen = navLinks.classList.toggle("open");
            navToggle.innerHTML = isOpen ? "&times;" : "&#9776;";
            // Keep font size consistent when changing character
            if (isOpen) {
                navToggle.style.fontSize = "2rem";
                navToggle.style.lineHeight = "1";
            } else {
                navToggle.style.fontSize = "";
                navToggle.style.lineHeight = "";
            }
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

    // ── Hero Visual Parallax Animation ─────────────────────────────

    var heroVisual = document.getElementById("hero-visual");
    if (heroVisual) {
        window.addEventListener("scroll", function () {
            // Get the geometry
            var rect = heroVisual.getBoundingClientRect();
            var windowHeight = window.innerHeight;

            // Calculate progress: 0 when top enters view, 1 when it hits the top third of the screen
            // The image starts out tilted and gets flat/normal as it gets to the center
            var startY = windowHeight;
            var endY = windowHeight * 0.3;

            var progress = 0;
            if (rect.top <= endY) {
                progress = 1;
            } else if (rect.top < startY) {
                progress = 1 - ((rect.top - endY) / (startY - endY));
            }

            // Clamp progress between 0 and 1
            progress = Math.max(0, Math.min(1, progress));

            // Apply it as a Custom Property for CSS to use
            heroVisual.style.setProperty("--scroll-progress", progress);
        }, { passive: true });

        // Trigger once on load to set initial state
        window.dispatchEvent(new Event("scroll"));
    }

    // ── Script Viewer Modal ──────────────────────────────────────

    var scriptModal = document.getElementById("script-modal");
    var scriptContent = document.getElementById("script-content");
    var modalTitle = document.getElementById("modal-title");
    var viewScriptBtn = document.getElementById("view-script-btn");
    var modalClose = scriptModal ? scriptModal.querySelector(".modal-close") : null;
    var modalCopyBtn = document.getElementById("modal-copy-btn");

    if (scriptModal && viewScriptBtn) {
        viewScriptBtn.addEventListener("click", function () {
            var activeTab = document.querySelector(".install-tab.active");
            var scriptName = "install.sh"; // Default

            if (activeTab) {
                var targetId = activeTab.getAttribute("data-target");
                if (targetId === "cmd-windows") {
                    scriptName = "install.ps1";
                } else {
                    scriptName = "install.sh";
                }
            }

            modalTitle.textContent = "Script Preview: " + scriptName;
            scriptModal.classList.add("active");
            scriptContent.textContent = "Loading script...";
            document.body.style.overflow = "hidden";

            var githubUrl = "https://raw.githubusercontent.com/youssefsz/sys-monitor-Rust/master/" + scriptName;

            fetch(githubUrl)
                .then(function (response) {
                    if (!response.ok) throw new Error("Could not load script");
                    return response.text();
                })
                .then(function (text) {
                    scriptContent.textContent = text;
                })
                .catch(function () {
                    scriptContent.textContent = "Error: Failed to load script content.\n\nYou can view it directly on GitHub:\nhttps://github.com/youssefsz/sys-monitor-Rust/blob/master/" + scriptName;
                });
        });

        if (modalClose) {
            modalClose.addEventListener("click", function () {
                scriptModal.classList.remove("active");
                document.body.style.overflow = "";
            });
        }

        // Close on clic outside
        scriptModal.addEventListener("click", function (e) {
            if (e.target === scriptModal) {
                scriptModal.classList.remove("active");
                document.body.style.overflow = "";
            }
        });

        // Modal copy button
        if (modalCopyBtn) {
            modalCopyBtn.addEventListener("click", function () {
                var text = scriptContent.textContent;
                navigator.clipboard.writeText(text).then(function () {
                    modalCopyBtn.textContent = "Copied!";
                    setTimeout(function () {
                        modalCopyBtn.textContent = "Copy Script";
                    }, 2000);
                });
            });
        }
    }
})();
