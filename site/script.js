/* CogWasm site enhancements.
 *
 * Two small things only: the phone menu and the live benchmark headline.
 * Without JavaScript the navigation stays visible and the benchmark section
 * keeps its static "see the chart" text and image. Nothing else depends on
 * this file.
 */
(function () {
  "use strict";

  document.documentElement.classList.add("js");

  var toggle = document.querySelector(".menu-toggle");
  var nav = document.getElementById("site-nav");

  function setMenu(open) {
    if (!toggle || !nav) {
      return;
    }
    toggle.setAttribute("aria-expanded", open ? "true" : "false");
    toggle.textContent = open ? "Close" : "Menu";
    nav.classList.toggle("is-open", open);
  }

  if (toggle && nav) {
    toggle.addEventListener("click", function () {
      setMenu(toggle.getAttribute("aria-expanded") !== "true");
    });

    nav.addEventListener("click", function (event) {
      var target = event.target;
      if (target && target.closest && target.closest("a")) {
        setMenu(false);
      }
    });

    document.addEventListener("keydown", function (event) {
      if (event.key === "Escape" && toggle.getAttribute("aria-expanded") === "true") {
        setMenu(false);
        toggle.focus();
      }
    });

    window.addEventListener("resize", function () {
      if (window.innerWidth > 800) {
        setMenu(false);
      }
    });
  }

  var live = document.getElementById("bench-live");
  if (!live || typeof window.fetch !== "function") {
    return;
  }

  function element(tag, className, text) {
    var node = document.createElement(tag);
    if (className) {
      node.className = className;
    }
    if (text !== undefined) {
      node.textContent = text;
    }
    return node;
  }

  function ratio(value) {
    return Number(value).toFixed(2) + "\u00d7";
  }

  function millis(value) {
    return Number(value).toFixed(1) + " ms";
  }

  function programCount(value) {
    return typeof value === "number" && value > 0 ? " over " + value + " programs" : "";
  }

  function metric(label, value, note) {
    var item = element("div", "live-item");
    item.appendChild(element("span", "live-label", label));
    item.appendChild(element("span", "live-value", value));
    item.appendChild(element("span", "live-note", note));
    return item;
  }

  window
    .fetch("bench/data.json", { cache: "no-store" })
    .then(function (response) {
      if (!response.ok) {
        throw new Error("HTTP " + response.status);
      }
      return response.json();
    })
    .then(function (data) {
      if (!data || !Array.isArray(data.points) || data.points.length === 0) {
        return;
      }

      var point = data.points[data.points.length - 1];
      var metrics = point.metrics || {};
      var values = [];

      if (typeof metrics.exec_ratio === "number") {
        values.push(
          metric("Execution", ratio(metrics.exec_ratio), "LLVM -O2 geomean" + programCount(point.exec_programs))
        );
      }
      if (typeof metrics.size_ratio === "number") {
        values.push(
          metric("Module size", ratio(metrics.size_ratio), "LLVM -O2 geomean" + programCount(point.size_programs))
        );
      }
      if (typeof metrics.warm_compile_ms === "number") {
        values.push(metric("Warm compile", millis(metrics.warm_compile_ms), "median warm compile"));
      }
      if (values.length === 0) {
        return;
      }

      live.textContent = "";
      var grid = element("div", "live-grid");
      values.forEach(function (item) {
        grid.appendChild(item);
      });
      live.appendChild(grid);

      var meta = element("p", "live-meta");
      meta.appendChild(document.createTextNode("Latest CI point "));

      var sha = String(point.sha || "");
      if (/^[0-9a-f]{7,40}$/.test(sha)) {
        var commit = element("a", "", point.short_sha || sha.slice(0, 7));
        commit.href = "https://github.com/TeamNodeCraft/CogWasm/commit/" + sha;
        meta.appendChild(commit);
      } else {
        meta.appendChild(document.createTextNode("unknown commit"));
      }

      var date = String(point.date || "").slice(0, 10);
      if (/^\d{4}-\d{2}-\d{2}$/.test(date)) {
        var time = element("time", "", date);
        time.dateTime = date;
        meta.appendChild(document.createTextNode(", "));
        meta.appendChild(time);
      }

      if (point.runner) {
        meta.appendChild(document.createTextNode(" on " + point.runner));
      }
      meta.appendChild(document.createTextNode(". "));
      var chart = element("a", "", "Interactive chart");
      chart.href = "bench/";
      meta.appendChild(chart);
      live.appendChild(meta);
    })
    .catch(function () {
      // The data may be absent or the page may be opened from disk. The static
      // fallback and the chart image are the intended path in that case.
    });
})();
