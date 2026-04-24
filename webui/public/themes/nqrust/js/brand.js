(function () {
  "use strict";

  var replacements = [
    [/NQRustBackup WebUI/gi, "NQRust Backup Console"],
    [/NQRust Backup Web Console/gi, "NQRust Backup Console"],
    [/NQRustBackup Director Version Information/gi, "NQRust Backup Director Status"],
    [/NQRustBackup Director/gi, "NQRust Backup Director"],
    [/NQRustBackup Tray Monitor/gi, "NQRust Backup Monitor"],
    [/nqrustbackup-dir/gi, "nqrust-director"],
    [/nqrustbackup-fd/gi, "nqrust-client"],
    [/nqrustbackup-sd/gi, "nqrust-storage"],
    [/nqrustbackup-webui/gi, "nqrust-backup"],
    [/NQRustBackup GmbH\s*&amp;\s*Co\. KG/gi, "NQRust"],
    [/NQRustBackup GmbH\s*&\s*Co\. KG/gi, "NQRust"],
    [/NQRustBackup/gi, "NQRust Backup"],
    [/nqrustbackup/gi, "nqrust-backup"],
    [/NQRUSTBACKUP/g, "NQRUST BACKUP"]
  ];

  function replaceBranding(value) {
    if (!value || typeof value !== "string") {
      return value;
    }

    var result = value;
    replacements.forEach(function (item) {
      result = result.replace(item[0], item[1]);
    });
    return result;
  }

  function scrubNode(node) {
    if (!node) {
      return;
    }

    if (node.nodeType === Node.TEXT_NODE) {
      var next = replaceBranding(node.nodeValue);
      if (next !== node.nodeValue) {
        node.nodeValue = next;
      }
      return;
    }

    if (node.nodeType !== Node.ELEMENT_NODE) {
      return;
    }

    var tagName = node.tagName.toLowerCase();
    if (tagName === "script" || tagName === "style" || tagName === "textarea") {
      return;
    }

    [
      "title",
      "alt",
      "placeholder",
      "aria-label",
      "href",
      "action",
      "value",
      "data-url",
      "data-href"
    ].forEach(function (name) {
      if (node.hasAttribute(name)) {
        var value = node.getAttribute(name);
        var next = replaceBranding(value);
        if (next !== value) {
          node.setAttribute(name, next);
        }
      }
    });

    Array.prototype.forEach.call(node.childNodes, scrubNode);
  }

  function scrubDocument() {
    document.title = replaceBranding(document.title);
    scrubNode(document.body);
    if (/nqrustbackup/i.test(window.location.href)) {
      window.history.replaceState(
        window.history.state,
        document.title,
        replaceBranding(window.location.href)
      );
    }
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", scrubDocument);
  } else {
    scrubDocument();
  }

  if (window.jQuery) {
    window.jQuery(document).ajaxComplete(function () {
      window.setTimeout(scrubDocument, 25);
    });
  }

  if (window.MutationObserver) {
    var pending = false;
    var observer = new MutationObserver(function () {
      if (pending) {
        return;
      }
      pending = true;
      window.requestAnimationFrame(function () {
        pending = false;
        scrubDocument();
      });
    });

    document.addEventListener("DOMContentLoaded", function () {
      observer.observe(document.body, {
        childList: true,
        subtree: true,
        characterData: true
      });
    });
  }
})();
