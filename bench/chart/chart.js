// Interactive benchmark trend chart for the Benchmarks section of the CogWasm site.
// Reads data.json (built by bench/chart/chart.py from the bench-history release)
// from the URL in the container's data-src attribute. No libraries.
(function () {
  "use strict";

  var SVG_NS = "http://www.w3.org/2000/svg";
  var FONT = "-apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";
  var CSS_COLOR = {
    exec_ratio: "--s-exec",
    exec_instructions_ratio: "--s-instr",
    size_ratio: "--s-size",
    warm_compile_ms: "--s-warm",
    incremental_ms: "--s-inc",
    precompiled_load_ms: "--s-pre",
    compiler_instructions: "--s-fuel"
  };
  var state = { data: null, hidden: new Set(), touchPoint: null, suppressClick: false };
  var tooltip = document.getElementById("bench-tooltip");
  var chart = document.getElementById("bench-chart");
  var hideTimer = null;

  function svgEl(name, attrs, parent) {
    var node = document.createElementNS(SVG_NS, name);
    if (attrs) {
      Object.keys(attrs).forEach(function (key) {
        node.setAttribute(key, attrs[key]);
      });
    }
    if (parent) { parent.appendChild(node); }
    return node;
  }

  function svgText(parent, x, y, text, attrs) {
    var node = svgEl("text", Object.assign({
      x: x,
      y: y,
      "font-family": FONT,
      "font-size": 11,
      fill: "var(--muted)"
    }, attrs || {}), parent);
    node.textContent = text;
    return node;
  }

  function colorFor(key) {
    return "var(" + (CSS_COLOR[key] || "--muted") + ")";
  }

  function marker(parent, kind, x, y, color, size) {
    size = size || 3.4;
    var node;
    if (kind === "circle") {
      node = svgEl("circle", { cx: x, cy: y, r: size, fill: color, stroke: "var(--panel)", "stroke-width": 1 }, parent);
    } else if (kind === "square") {
      node = svgEl("rect", { x: x - size, y: y - size, width: size * 2, height: size * 2, rx: 0.5, fill: color, stroke: "var(--panel)", "stroke-width": 1 }, parent);
    } else if (kind === "triangle") {
      node = svgEl("path", { d: "M " + x + " " + (y - size) + " L " + (x + size) + " " + (y + size) + " L " + (x - size) + " " + (y + size) + " Z", fill: color, stroke: "var(--panel)", "stroke-width": 1 }, parent);
    } else if (kind === "diamond") {
      node = svgEl("path", { d: "M " + x + " " + (y - size) + " L " + (x + size) + " " + y + " L " + x + " " + (y + size) + " L " + (x - size) + " " + y + " Z", fill: color, stroke: "var(--panel)", "stroke-width": 1 }, parent);
    } else if (kind === "cross") {
      node = svgEl("path", { d: "M " + (x - size) + " " + (y - size) + " L " + (x + size) + " " + (y + size) + " M " + (x + size) + " " + (y - size) + " L " + (x - size) + " " + (y + size), stroke: color, "stroke-width": 1.9, "stroke-linecap": "round" }, parent);
    } else if (kind === "star") {
      var points = [];
      for (var index = 0; index < 10; index += 1) {
        var angle = -Math.PI / 2 + index * Math.PI / 5;
        var radius = index % 2 === 0 ? size : size * 0.45;
        points.push((x + Math.cos(angle) * radius).toFixed(2) + "," + (y + Math.sin(angle) * radius).toFixed(2));
      }
      node = svgEl("polygon", { points: points.join(" "), fill: color, stroke: "var(--panel)", "stroke-width": 0.8 }, parent);
    } else {
      node = svgEl("path", { d: "M " + (x - size) + " " + y + " L " + (x + size) + " " + y + " M " + x + " " + (y - size) + " L " + x + " " + (y + size), stroke: color, "stroke-width": 1.9, "stroke-linecap": "round" }, parent);
    }
    return node;
  }

  function legendMarker(kind, color) {
    var svg = document.createElementNS(SVG_NS, "svg");
    svg.setAttribute("width", "13");
    svg.setAttribute("height", "13");
    svg.setAttribute("viewBox", "0 0 13 13");
    svg.setAttribute("aria-hidden", "true");
    marker(svg, kind, 6.5, 6.5, color, 3.2);
    return svg;
  }

  function escapeHtml(value) {
    return String(value).replace(/[&<>"']/g, function (character) {
      return { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[character];
    });
  }

  function formatDate(iso) {
    var date = new Date(iso);
    if (isNaN(date.getTime())) { return iso; }
    return date.toISOString().slice(0, 10);
  }

  function formatCount(value) {
    var units = ["", "k", "M", "G", "T"];
    var scaled = Number(value);
    var unit = 0;
    while (Math.abs(scaled) >= 1000 && unit < units.length - 1) {
      scaled /= 1000;
      unit += 1;
    }
    var digits = scaled >= 100 || unit === 0 ? 0 : 1;
    return scaled.toFixed(digits).replace(/\.0$/, "") + units[unit];
  }

  function formatValue(series, value) {
    if (series.unit === "instructions") {
      return formatCount(value) + " instructions";
    }
    if (series.unit === "ms") {
      return Number(value).toFixed(series.digits) + " ms";
    }
    return Number(value).toFixed(series.digits) + "× LLVM -O2";
  }

  function seriesHasData(series) {
    return state.data.points.some(function (point) {
      return point.metrics[series.key] !== null && point.metrics[series.key] !== undefined;
    });
  }

  function visibleSeries(panel) {
    return state.data.series.filter(function (series) {
      return series.panel === panel && !state.hidden.has(series.key);
    });
  }

  function buildLegend() {
    var legend = document.getElementById("bench-legend");
    legend.textContent = "";
    state.data.series.forEach(function (series) {
      var button = document.createElement("button");
      var hasData = seriesHasData(series);
      button.type = "button";
      button.setAttribute("aria-pressed", state.hidden.has(series.key) ? "false" : "true");
      button.disabled = !hasData;
      button.title = hasData ? "Hide or show this line" : "No values for this line yet";
      button.appendChild(legendMarker(series.marker, colorFor(series.key)));
      var label = document.createElement("span");
      label.textContent = series.label + (hasData ? "" : " (no data yet)");
      button.appendChild(label);
      button.addEventListener("click", function () {
        if (state.hidden.has(series.key)) {
          state.hidden.delete(series.key);
        } else {
          state.hidden.add(series.key);
        }
        hideTooltip();
        buildLegend();
        drawChart();
      });
      legend.appendChild(button);
    });
    var reference = document.createElement("span");
    reference.className = "reference";
    reference.innerHTML = '<svg width="18" height="13" viewBox="0 0 18 13" aria-hidden="true">' +
      '<line x1="1" y1="6.5" x2="17" y2="6.5" stroke="var(--muted)" stroke-width="1.4" stroke-dasharray="3 2"/></svg>' +
      "<span>LLVM -O2 parity</span>";
    legend.appendChild(reference);
    var note = document.createElement("span");
    note.className = "legend-note";
    note.textContent = "Solid = deterministic instruction count · dotted = wall-clock";
    legend.appendChild(note);
  }

  function niceTicks(high, intervals) {
    intervals = intervals || 5;
    high = Math.max(high, 1e-9);
    var rawStep = high / intervals;
    var exponent = Math.floor(Math.log10(rawStep));
    var fraction = rawStep / Math.pow(10, exponent);
    var candidates = [1, 1.5, 2, 2.5, 3, 4, 5, 7.5, 10];
    var step = 10;
    for (var index = 0; index < candidates.length; index += 1) {
      if (fraction <= candidates[index]) {
        step = candidates[index];
        break;
      }
    }
    step *= Math.pow(10, exponent);
    var top = Math.ceil(high / step - 1e-9) * step;
    var ticks = [];
    var count = Math.round(top / step);
    for (var tick = 0; tick <= count; tick += 1) {
      ticks.push(tick * step);
    }
    return { top: top, ticks: ticks };
  }

  function panelHigh(panel, axis) {
    axis = axis || "left";
    var values = [];
    state.data.points.forEach(function (point) {
      state.data.series.forEach(function (series) {
        if (series.panel === panel && (series.axis || "left") === axis && point.metrics[series.key] !== null && point.metrics[series.key] !== undefined) {
          values.push(point.metrics[series.key]);
        }
      });
    });
    var high = values.length ? Math.max.apply(null, values) * 1.06 : 1;
    if (panel === state.data.reference.panel && axis === "left") {
      // Keep the parity line on the panel once the ratios drop below 1.0.
      high = Math.max(high, state.data.reference.value * 1.05);
    }
    return high;
  }

  function xPositions(count, left, right) {
    if (count <= 1) { return [(left + right) / 2]; }
    var step = (right - left) / (count - 1);
    return state.data.points.map(function (_, index) { return left + index * step; });
  }

  function segmentsFor(points, key, xs, yFor) {
    var segments = [];
    var current = [];
    points.forEach(function (point, index) {
      var value = point.metrics[key];
      if (value === null || value === undefined) {
        if (current.length > 1) { segments.push(current); }
        current = [];
      } else {
        current.push([xs[index], yFor(value)]);
      }
    });
    if (current.length > 1) { segments.push(current); }
    return segments;
  }

  function drawChart() {
    if (!state.data) { return; }
    var width = Math.max(320, Math.round(chart.clientWidth || 800));
    var stacked = width < 680;
    var margin = stacked ? 4 : 6;
    var gap = stacked ? 14 : 20;
    var panelWidth = stacked ? width - 2 * margin : (width - 2 * margin - gap) / 2;
    var panelHeight = stacked ? 330 : 404;
    var height = stacked ? panelHeight * 2 + gap : panelHeight;
    var plotTop = 52;
    var plotBottom = panelHeight - 86;
    var labelStep = Math.max(1, Math.ceil(state.data.points.length / (stacked ? 5 : 9)));
    var labelIndices = new Set();
    state.data.points.forEach(function (_, index) {
      if (index % labelStep === 0) { labelIndices.add(index); }
    });
    if (state.data.points.length) { labelIndices.add(state.data.points.length - 1); }

    var svg = svgEl("svg", {
      viewBox: "0 0 " + width + " " + height,
      role: "img",
      "aria-label": "CogWasm benchmark trend by commit. Two panels: execution time, executed instructions and module size against LLVM -O2, and compile speed plus compiler instructions. Lower is better. Solid lines are deterministic instruction counts; dotted lines are wall-clock."
    });
    state.data.panels.forEach(function (panel, panelIndex) {
      var panelX = stacked ? margin : margin + panelIndex * (panelWidth + gap);
      var panelY = stacked ? panelIndex * (panelHeight + gap) : 0;
      var plotLeft = panelX + (stacked ? 48 : 50);
      var rightSeries = state.data.series.find(function (series) {
        return series.panel === panel.id && (series.axis || "left") === "right" && seriesHasData(series);
      });
      var plotRight = panelX + panelWidth - (rightSeries ? (stacked ? 44 : 50) : 14);
      var xs = xPositions(state.data.points.length, plotLeft, plotRight);
      var geometry = niceTicks(panelHigh(panel.id, "left"));
      var top = geometry.top;
      var ticks = geometry.ticks;
      var rightGeometry = rightSeries ? niceTicks(panelHigh(panel.id, "right")) : null;
      var rightTop = rightGeometry ? rightGeometry.top : top;
      var yFor = function (value) {
        return panelY + plotBottom - (value / top) * (plotBottom - plotTop);
      };
      var yForRight = function (value) {
        return panelY + plotBottom - (value / rightTop) * (plotBottom - plotTop);
      };

      svgEl("rect", {
        x: panelX, y: panelY, width: panelWidth, height: panelHeight,
        rx: 10, fill: "var(--panel)", stroke: "var(--border)"
      }, svg);
      svgText(svg, panelX + 12, panelY + 26, panel.title, {
        "font-size": stacked ? 12 : 12.5,
        "font-weight": 600,
        fill: "var(--ink)"
      });

      ticks.forEach(function (tick) {
        var y = yFor(tick);
        svgEl("line", { x1: plotLeft, y1: y, x2: plotRight, y2: y, stroke: "var(--grid)", "stroke-width": 1 }, svg);
        svgText(svg, plotLeft - 7, y + 3, String(Math.round(tick * 1000) / 1000), {
          "font-size": stacked ? 10 : 10.5,
          "text-anchor": "end"
        });
      });
      if (rightGeometry) {
        rightGeometry.ticks.forEach(function (tick) {
          var y = yForRight(tick);
          svgEl("line", { x1: plotRight, y1: y, x2: plotRight + 3, y2: y, stroke: "var(--border)", "stroke-width": 1 }, svg);
          svgText(svg, plotRight + 6, y + 3, formatCount(tick), {
            "font-size": stacked ? 9 : 9.5,
            "text-anchor": "start",
            fill: colorFor(rightSeries.key)
          });
        });
        svgText(svg, plotRight + 6, panelY + plotTop - 8, "instructions", {
          "font-size": 8.5,
          "text-anchor": "start",
          fill: colorFor(rightSeries.key)
        });
      }
      svgEl("line", { x1: plotLeft, y1: panelY + plotBottom, x2: plotRight, y2: panelY + plotBottom, stroke: "var(--border)", "stroke-width": 1 }, svg);

      state.data.points.forEach(function (point, index) {
        if (!labelIndices.has(index)) { return; }
        var x = xs[index];
        svgText(svg, x, panelY + plotBottom + 14, point.short_sha, {
          "font-size": stacked ? 9.5 : 10,
          "text-anchor": "end",
          transform: "rotate(-40 " + x + " " + (panelY + plotBottom + 14) + ")",
          fill: point.subset_changed ? "var(--accent)" : "var(--muted)",
          "font-weight": point.subset_changed ? 600 : 400
        });
      });

      state.data.points.forEach(function (point, index) {
        if (!point.subset_changed) { return; }
        var x = xs[index];
        var group = svgEl("g", {}, svg);
        var title = svgEl("title", {}, group);
        var changes = [];
        if (point.subset_added.length) { changes.push("added " + point.subset_added.join(", ")); }
        if (point.subset_removed.length) { changes.push("removed " + point.subset_removed.join(", ")); }
        title.textContent = "Quick program subset changed at " + point.short_sha +
          " (" + point.program_count + " programs): " + (changes.join("; ") || "program list changed");
        svgEl("line", {
          x1: x, y1: panelY + plotTop, x2: x, y2: panelY + plotBottom,
          stroke: "var(--accent)", "stroke-width": 1, "stroke-dasharray": "2 3", opacity: 0.75
        }, group);
        svgEl("path", {
          d: "M " + (x - 3.5) + " " + (panelY + plotTop - 5) + " L " + (x + 3.5) + " " +
            (panelY + plotTop - 5) + " L " + x + " " + (panelY + plotTop + 2) + " Z",
          fill: "var(--accent)"
        }, group);
        var hit = svgEl("rect", {
          x: x - 7, y: panelY + plotTop - 8, width: 14, height: plotBottom - plotTop + 8,
          fill: "transparent", "pointer-events": "all", "cursor": "help"
        }, group);
        hit.addEventListener("pointerenter", function (event) {
          if (event.pointerType !== "touch") { showSubset(point, event); }
        });
        hit.addEventListener("pointerdown", function (event) {
          if (event.pointerType === "touch") { showSubset(point, event); }
        });
        hit.addEventListener("pointerleave", hideTooltipSoon);
      });

      if (panel.id === 1) {
        var referenceY = yFor(state.data.reference.value);
        svgEl("line", {
          x1: plotLeft, y1: referenceY, x2: plotRight, y2: referenceY,
          stroke: "var(--muted)", "stroke-width": 1, "stroke-dasharray": "4 3"
        }, svg);
        svgText(svg, plotRight - 2, referenceY - 4, state.data.reference.label, {
          "text-anchor": "end", "font-size": 9.5
        });
      }

      state.data.series.forEach(function (series) {
        if (series.panel !== panel.id || state.hidden.has(series.key)) { return; }
        var color = colorFor(series.key);
        var valueY = (series.axis || "left") === "right" ? yForRight : yFor;
        var dash = series.deterministic ? null : "5 3";
        segmentsFor(state.data.points, series.key, xs, valueY).forEach(function (segment) {
          var attributes = {
            points: segment.map(function (pair) { return pair[0] + "," + pair[1]; }).join(" "),
            fill: "none", stroke: color, "stroke-width": 1.9,
            "stroke-linejoin": "round", "stroke-linecap": "round"
          };
          if (dash) { attributes["stroke-dasharray"] = dash; }
          svgEl("polyline", attributes, svg);
        });
        state.data.points.forEach(function (point, index) {
          var value = point.metrics[series.key];
          if (value === null || value === undefined) { return; }
          marker(svg, series.marker, xs[index], valueY(value), color, 3.4);
          var hit = svgEl("circle", {
            cx: xs[index], cy: valueY(value), r: 12, fill: "transparent",
            "pointer-events": "all", "cursor": "pointer", tabindex: 0,
            role: "button",
            "aria-label": point.short_sha + " " + series.label
          }, svg);
          bindPoint(hit, point);
        });
      });
    });
    chart.textContent = "";
    chart.appendChild(svg);
  }

  function bindPoint(node, point) {
    node.addEventListener("pointerenter", function (event) {
      if (event.pointerType !== "touch") { showPoint(point, event); }
    });
    node.addEventListener("pointerdown", function (event) {
      if (event.pointerType === "touch" || window.matchMedia("(pointer: coarse)").matches) {
        state.suppressClick = true;
        window.setTimeout(function () { state.suppressClick = false; }, 500);
        if (state.touchPoint === point.index) {
          openCommit(point);
        } else {
          state.touchPoint = point.index;
          showPoint(point, event);
          event.preventDefault();
        }
      } else {
        showPoint(point, event);
      }
    });
    node.addEventListener("click", function (event) {
      if (state.suppressClick) {
        state.suppressClick = false;
        event.preventDefault();
        return;
      }
      openCommit(point);
    });
    node.addEventListener("focus", function () {
      showPoint(point, { clientX: window.innerWidth / 2, clientY: 120 });
    });
    node.addEventListener("keydown", function (event) {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        openCommit(point);
      }
    });
    node.addEventListener("blur", hideTooltipSoon);
    node.addEventListener("pointerleave", hideTooltipSoon);
  }

  function openCommit(point) {
    var repo = state.data.repo;
    window.open("https://github.com/" + repo + "/commit/" + point.sha, "_blank", "noopener");
  }

  function tooltipHeader(point) {
    return '<div class="head"><span class="sha">' + escapeHtml(point.short_sha) +
      '</span><span class="date">' + escapeHtml(formatDate(point.date)) +
      '</span></div><div class="subject">' + escapeHtml(point.subject) + "</div>";
  }

  function tooltipMeta(point) {
    var run = point.run_id ? " · run " + escapeHtml(point.run_id) : "";
    return '<div class="meta">' + point.program_count + " quick programs" +
      escapeHtml(run) + " · " + escapeHtml(point.runner) + "</div>";
  }

  function tooltipRows(point) {
    var rows = "";
    state.data.series.forEach(function (series) {
      if (state.hidden.has(series.key)) { return; }
      var value = point.metrics[series.key];
      if (value === null || value === undefined) { return; }
      rows += '<tr><td><span class="swatch" style="background:' + colorFor(series.key) +
        '"></span>' + escapeHtml(series.label) + "</td><td class=\"value\">" +
        escapeHtml(formatValue(series, value)) + "</td></tr>";
    });
    return rows ? "<table>" + rows + "</table>" : "";
  }

  function noteFor(point) {
    var notes = [];
    if (point.metrics.exec_ratio !== null && point.metrics.exec_ratio !== undefined) {
      notes.push("execution ratio over " + point.exec_programs +
        " programs (LLVM time ≥ " + state.data.exec_min_llvm_ms + " ms)");
    }
    if (point.metrics.exec_instructions_ratio !== null && point.metrics.exec_instructions_ratio !== undefined) {
      notes.push("executed-instruction ratio over " + (point.instruction_programs || 0) +
        " programs, same time floor");
    }
    if (point.metrics.compiler_instructions !== null && point.metrics.compiler_instructions !== undefined) {
      notes.push("compiler instructions: median of " + (point.compiler_programs || 0) +
        " programs, deterministic fuel");
    }
    if (point.metrics.size_ratio !== null && point.metrics.size_ratio !== undefined) {
      notes.push("module size over " + point.size_programs + " programs");
    }
    if (point.subset_changed) {
      var changes = [];
      if (point.subset_added.length) { changes.push("+" + point.subset_added.join(", +")); }
      if (point.subset_removed.length) { changes.push("−" + point.subset_removed.join(", −")); }
      notes.push("subset changed: " + changes.join("; "));
    }
    return notes.length ? '<div class="note">' + escapeHtml(notes.join(" · ")) + "</div>" : "";
  }

  function showPoint(point, event) {
    cancelHide();
    var hint = window.matchMedia("(pointer: coarse)").matches
      ? '<div class="note">Tap the point again to open the commit.</div>'
      : "";
    tooltip.innerHTML = tooltipHeader(point) + tooltipMeta(point) + tooltipRows(point) + noteFor(point) + hint;
    tooltip.hidden = false;
    placeTooltip(event);
  }

  function showSubset(point, event) {
    cancelHide();
    var changes = [];
    if (point.subset_added.length) { changes.push("added " + point.subset_added.join(", ")); }
    if (point.subset_removed.length) { changes.push("removed " + point.subset_removed.join(", ")); }
    tooltip.innerHTML = tooltipHeader(point) +
      '<div class="note subset">Quick program subset changed vs the previous commit: ' +
      escapeHtml(changes.join("; ") || "program list changed") + " (" +
      point.program_count + " programs).</div>";
    tooltip.hidden = false;
    placeTooltip(event);
  }

  function placeTooltip(event) {
    var pad = 10;
    var x = event && event.clientX ? event.clientX : window.innerWidth / 2;
    var y = event && event.clientY ? event.clientY : 120;
    tooltip.style.left = "0px";
    tooltip.style.top = "0px";
    var rect = tooltip.getBoundingClientRect();
    var left = x + 14;
    var top = y + 14;
    if (left + rect.width > window.innerWidth - pad) { left = x - rect.width - 14; }
    if (top + rect.height > window.innerHeight - pad) { top = y - rect.height - 14; }
    left = Math.max(pad, Math.min(left, window.innerWidth - rect.width - pad));
    top = Math.max(pad, Math.min(top, window.innerHeight - rect.height - pad));
    tooltip.style.left = left + "px";
    tooltip.style.top = top + "px";
  }

  function hideTooltip() {
    tooltip.hidden = true;
    state.touchPoint = null;
  }

  function hideTooltipSoon() {
    cancelHide();
    hideTimer = window.setTimeout(hideTooltip, 180);
  }

  function cancelHide() {
    if (hideTimer) {
      window.clearTimeout(hideTimer);
      hideTimer = null;
    }
  }

  function render() {
    buildLegend();
    drawChart();
    var repo = state.data.repo;
    var footnote = document.getElementById("bench-footnote");
    footnote.innerHTML = "Source: <a href=\"https://github.com/" + escapeHtml(repo) +
      "/tree/main/bench/chart\">bench/chart</a> · " +
      "one point per successful quick run on main, from the append-only " +
      "<code>bench-history</code> release. Instruction lines are exact counts " +
      "from basic-block instrumentation; compiler work is Wasmtime fuel. " +
      "Static image for the README: " +
      "<a href=\"https://github.com/" + escapeHtml(repo) +
      "/releases/download/bench-history/chart.svg\">chart.svg</a>.";
  }

  function fail(message) {
    chart.innerHTML = '<div class="message">' + escapeHtml(message) + "</div>";
  }

  document.addEventListener("pointerdown", function (event) {
    var target = event.target;
    if (!target || !target.closest) { return; }
    if (!target.closest(".chart") && !target.closest(".tooltip")) {
      hideTooltip();
    }
  });

  window.addEventListener("resize", function () {
    hideTooltip();
    if (state.data) { drawChart(); }
  });

  fetch(chart.getAttribute("data-src") || "bench/data.json", { cache: "no-store" })
    .then(function (response) {
      if (!response.ok) { throw new Error("HTTP " + response.status); }
      return response.json();
    })
    .then(function (data) {
      state.data = data;
      if (!data.points || !data.points.length) {
        fail("No CI benchmark points have been published yet.");
        return;
      }
      render();
    })
    .catch(function (error) {
      fail("Could not load data.json (" + error.message + "). If you opened the page from disk, serve the built site over HTTP instead.");
    });
})();
