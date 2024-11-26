  const BRAND_SYMBOL = Symbol.for("preact-signals");
  const RUNNING = 1 << 0;
  const NOTIFIED = 1 << 1;
  const OUTDATED = 1 << 2;
  const DISPOSED = 1 << 3;
  const HAS_ERROR = 1 << 4;
  const TRACKING = 1 << 5;

  let batchedEffect;
  let batchDepth = 0;
  let batchIteration = 0;
  let globalVersion = 0;
  let evalContext;

  function startBatch() {
    batchDepth++;
  }

  function endBatch() {
    if (batchDepth > 1) {
      batchDepth--;
      return;
    }
    let error;
    let hasError = false;
    while (batchedEffect !== undefined) {
      let effect = batchedEffect;
      batchedEffect = undefined;
      batchIteration++;
      while (effect !== undefined) {
        let next = effect._nextBatchedEffect;
        effect._nextBatchedEffect = undefined;
        effect._flags &= ~NOTIFIED;
        if (!(effect._flags & DISPOSED) && needsToRecompute(effect)) {
          try {
            effect._callback();
          } catch (err) {
            if (!hasError) {
              error = err;
              hasError = true;
            }
          }
        }
        effect = next;
      }
    }
    batchIteration = 0;
    batchDepth--;
    if (hasError) {
      throw error;
    }
  }

  function batch(fn) {
    if (batchDepth > 0) {
      return fn();
    }
    startBatch();
    try {
      return fn();
    } finally {
      endBatch();
    }
  }

  function untracked(fn) {
    let prevContext = evalContext;
    evalContext = undefined;
    try {
      return fn();
    } finally {
      evalContext = prevContext;
    }
  }

  function addDependency(signal) {
    if (evalContext === undefined) {
      return undefined;
    }
    let node = signal._node;
    if (node === undefined || node._target !== evalContext) {
      node = {
        _version: 0,
        _source: signal,
        _prevSource: evalContext._sources,
        _nextSource: undefined,
        _target: evalContext,
        _prevTarget: undefined,
        _nextTarget: undefined,
        _rollbackNode: node,
      };
      if (evalContext._sources !== undefined) {
        evalContext._sources._nextSource = node;
      }
      evalContext._sources = node;
      signal._node = node;
      if (evalContext._flags & TRACKING) {
        signal._subscribe(node);
      }
      return node;
    } else if (node._version === -1) {
      node._version = 0;
      if (node._nextSource !== undefined) {
        node._nextSource._prevSource = node._prevSource;
        if (node._prevSource !== undefined) {
          node._prevSource._nextSource = node._nextSource;
        }
        node._prevSource = evalContext._sources;
        node._nextSource = undefined;
        evalContext._sources._nextSource = node;
        evalContext._sources = node;
      }
      return node;
    }
    return undefined;
  }

  function Signal(value) {
    this._value = value;
    this._version = 0;
    this._node = undefined;
    this._targets = undefined;
  }

  Signal.prototype.brand = BRAND_SYMBOL;
  Signal.prototype._refresh = function () {
    return true;
  };

  Signal.prototype._subscribe = function (node) {
    if (this._targets !== node && node._prevTarget === undefined) {
      node._nextTarget = this._targets;
      if (this._targets !== undefined) {
        this._targets._prevTarget = node;
      }
      this._targets = node;
    }
  };

  Signal.prototype._unsubscribe = function (node) {
    if (this._targets !== undefined) {
      let prev = node._prevTarget;
      let next = node._nextTarget;
      if (prev !== undefined) {
        prev._nextTarget = next;
        node._prevTarget = undefined;
      }
      if (next !== undefined) {
        next._prevTarget = prev;
        node._nextTarget = undefined;
      }
      if (node === this._targets) {
        this._targets = next;
      }
    }
  };

  Signal.prototype.subscribe = function (fn) {
    const _this = this;
    return effect(function () {
      let value = _this.value;
      let prevContext = evalContext;
      evalContext = undefined;
      try {
        fn(value);
      } finally {
        evalContext = prevContext;
      }
    });
  };

  Signal.prototype.valueOf = function () {
    return this.value;
  };

  Signal.prototype.toString = function () {
    return this.value + "";
  };

  Signal.prototype.toJSON = function () {
    return this.value;
  };

  Signal.prototype.peek = function () {
    let prevContext = evalContext;
    evalContext = undefined;
    try {
      return this.value;
    } finally {
      evalContext = prevContext;
    }
  };

  Object.defineProperty(Signal.prototype, "value", {
    get: function () {
      let node = addDependency(this);
      if (node !== undefined) {
        node._version = this._version;
      }
      return this._value;
    },
    set: function (value) {
      if (value !== this._value) {
        if (batchIteration > 100) {
          throw new Error("Cycle detected");
        }
        this._value = value;
        this._version++;
        globalVersion++;
        startBatch();
        try {
          for (
            let node = this._targets;
            node !== undefined;
            node = node._nextTarget
          ) {
            node._target._notify();
          }
        } finally {
          endBatch();
        }
      }
    },
  });

  function signal(value) {
    return new Signal(value);
  }

  function needsToRecompute(target) {
    for (
      let node = target._sources;
      node !== undefined;
      node = node._nextSource
    ) {
      if (
        node._source._version !== node._version ||
        !node._source._refresh() ||
        node._source._version !== node._version
      ) {
        return true;
      }
    }
    return false;
  }

  function prepareSources(target) {
    for (
      let node = target._sources;
      node !== undefined;
      node = node._nextSource
    ) {
      let rollbackNode = node._source._node;
      if (rollbackNode !== undefined) {
        node._rollbackNode = rollbackNode;
      }
      node._source._node = node;
      node._version = -1;
      if (node._nextSource === undefined) {
        target._sources = node;
        break;
      }
    }
  }

  function cleanupSources(target) {
    let node = target._sources;
    let head;
    while (node !== undefined) {
      let prev = node._prevSource;
      if (node._version === -1) {
        node._source._unsubscribe(node);
        if (prev !== undefined) {
          prev._nextSource = node._nextSource;
        }
        if (node._nextSource !== undefined) {
          node._nextSource._prevSource = prev;
        }
      } else {
        head = node;
      }
      node._source._node = node._rollbackNode;
      if (node._rollbackNode !== undefined) {
        node._rollbackNode = undefined;
      }
      node = prev;
    }
    target._sources = head;
  }

  function Computed(fn) {
    Signal.call(this, undefined);
    this._fn = fn;
    this._sources = undefined;
    this._globalVersion = globalVersion - 1;
    this._flags = OUTDATED;
  }

  Computed.prototype = new Signal();

  Computed.prototype._refresh = function () {
    this._flags &= ~NOTIFIED;
    if (this._flags & RUNNING) {
      return false;
    }
    if ((this._flags & (OUTDATED | TRACKING)) === TRACKING) {
      return true;
    }
    this._flags &= ~OUTDATED;
    if (this._globalVersion === globalVersion) {
      return true;
    }
    this._globalVersion = globalVersion;
    this._flags |= RUNNING;
    if (this._version > 0 && !needsToRecompute(this)) {
      this._flags &= ~RUNNING;
      return true;
    }
    let prevContext = evalContext;
    try {
      prepareSources(this);
      evalContext = this;
      let value = this._fn();
      if (
        this._flags & HAS_ERROR ||
        this._value !== value ||
        this._version === 0
      ) {
        this._value = value;
        this._flags &= ~HAS_ERROR;
        this._version++;
      }
    } catch (err) {
      this._value = err;
      this._flags |= HAS_ERROR;
      this._version++;
    }
    evalContext = prevContext;
    cleanupSources(this);
    this._flags &= ~RUNNING;
    return true;
  };

  Computed.prototype._subscribe = function (node) {
    if (this._targets === undefined) {
      this._flags |= OUTDATED | TRACKING;
      for (
        let node = this._sources;
        node !== undefined;
        node = node._nextSource
      ) {
        node._source._subscribe(node);
      }
    }
    Signal.prototype._subscribe.call(this, node);
  };

  Computed.prototype._unsubscribe = function (node) {
    if (this._targets !== undefined) {
      Signal.prototype._unsubscribe.call(this, node);
      if (this._targets === undefined) {
        this._flags &= ~TRACKING;
        for (
          let node = this._sources;
          node !== undefined;
          node = node._nextSource
        ) {
          node._source._unsubscribe(node);
        }
      }
    }
  };

  Computed.prototype._notify = function () {
    if (!(this._flags & NOTIFIED)) {
      this._flags |= OUTDATED | NOTIFIED;
      for (
        let node = this._targets;
        node !== undefined;
        node = node._nextTarget
      ) {
        node._target._notify();
      }
    }
  };

  Object.defineProperty(Computed.prototype, "value", {
    get: function () {
      if (this._flags & RUNNING) {
        throw new Error("Cycle detected");
      }
      let node = addDependency(this);
      this._refresh();
      if (node !== undefined) {
        node._version = this._version;
      }
      if (this._flags & HAS_ERROR) {
        throw this._value;
      }
      return this._value;
    },
  });

  function computed(fn) {
    return new Computed(fn);
  }

  function cleanupEffect(effect) {
    let cleanup = effect._cleanup;
    effect._cleanup = undefined;
    if (typeof cleanup === "function") {
      startBatch();
      let prevContext = evalContext;
      evalContext = undefined;
      try {
        cleanup();
      } catch (err) {
        effect._flags &= ~RUNNING;
        effect._flags |= DISPOSED;
        disposeEffect(effect);
        throw err;
      } finally {
        evalContext = prevContext;
        endBatch();
      }
    }
  }

  function disposeEffect(effect) {
    for (
      let node = effect._sources;
      node !== undefined;
      node = node._nextSource
    ) {
      node._source._unsubscribe(node);
    }
    effect._fn = undefined;
    effect._sources = undefined;
    cleanupEffect(effect);
  }

  function endEffect(prevContext) {
    if (evalContext !== this) {
      throw new Error("Out-of-order effect");
    }
    cleanupSources(this);
    evalContext = prevContext;
    this._flags &= ~RUNNING;
    if (this._flags & DISPOSED) {
      disposeEffect(this);
    }
    endBatch();
  }

  function Effect(fn) {
    this._fn = fn;
    this._cleanup = undefined;
    this._sources = undefined;
    this._nextBatchedEffect = undefined;
    this._flags = TRACKING;
  }

  Effect.prototype._callback = function () {
    let finish = this._start();
    try {
      if (this._flags & DISPOSED) return;
      if (this._fn === undefined) return;
      let cleanup = this._fn();
      if (typeof cleanup === "function") {
        this._cleanup = cleanup;
      }
    } finally {
      finish();
    }
  };

  Effect.prototype._start = function () {
    if (this._flags & RUNNING) {
      throw new Error("Cycle detected");
    }
    this._flags |= RUNNING;
    this._flags &= ~DISPOSED;
    cleanupEffect(this);
    prepareSources(this);
    startBatch();
    let prevContext = evalContext;
    evalContext = this;
    return endEffect.bind(this, prevContext);
  };

  Effect.prototype._notify = function () {
    if (!(this._flags & NOTIFIED)) {
      this._flags |= NOTIFIED;
      this._nextBatchedEffect = batchedEffect;
      batchedEffect = this;
    }
  };

  Effect.prototype._dispose = function () {
    this._flags |= DISPOSED;
    if (!(this._flags & RUNNING)) {
      disposeEffect(this);
    }
  };

  function effect(fn) {
    let effect = new Effect(fn);
    try {
      effect._callback();
    } catch (err) {
      effect._dispose();
      throw err;
    }
    return effect._dispose.bind(effect);
  }
  // signals.js - HTMX Extension for Preact Signals

  (function () {
    if (!window.htmx) {
      console.error("htmx must be loaded before this extension");
      return;
    }

    const stateRegistry = new Map();

    function initState(el) {
      const stateExpr = el.getAttribute("data-state");
      if (stateExpr) {
        const stateObj = new Function("return " + stateExpr)();
        for (const key in stateObj) {
          if (!stateRegistry.has(key)) {
            stateRegistry.set(key, signal(stateObj[key]));
          }
        }
        updateState(el, stateObj);
      }
    }

    function updateState(el, stateObj) {
      for (const key in stateObj) {
        if (stateRegistry.has(key)) {
          const sig = stateRegistry.get(key);
          sig.value = stateObj[key];
        }
      }
    }

    function bindState(el) {
      const stateBindings = el.getAttribute("data-bind");
      if (stateBindings) {
        const bindings = stateBindings.split(";");
        bindings.forEach((binding) => {
          const [stateKey, attr, event] = binding.split(":");
          const sig = stateRegistry.get(stateKey.trim());
          if (sig) {
            if (attr === "text") {
              el.textContent = sig.value;
              sig.subscribe(() => (el.textContent = sig.value));
            } else if (attr === "show") {
              el.style.display = sig.value ? "" : "none";
              sig.subscribe(() => (el.style.display = sig.value ? "" : "none"));
            }
            if (event) {
              el.addEventListener(event.trim(), () => {
                sig.value = !sig.value;
              });
            }
          }
        });
      }
    }

    htmx.defineExtension("signals", {
      onEvent: function (name, evt) {
        const target = evt.target;
        if (!target) return;

        if (name === "htmx:load" || name === "htmx:afterSwap") {
          target.querySelectorAll("[data-state]").forEach(initState);
          target.querySelectorAll("[data-bind]").forEach(bindState);
        }
      },
    });

    document.addEventListener("DOMContentLoaded", () => {
      document.querySelectorAll("[data-state]").forEach(initState);
      document.querySelectorAll("[data-bind]").forEach(bindState);
    });
  })();
