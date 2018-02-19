
"use strict";

// Adapted from https://github.com/badboy/hellorust/blob/master/demos/bundle.js

function copy_str(module, ptr, len) {
  let orig_ptr = ptr;

  const collect_str = function* () {
    let memory = new Uint8Array(module.memory.buffer);
    for (let i = 0; i < len; ++i) {
      let idx = ptr + i;
      if (memory[idx] === undefined) { throw new Error("Attempted to read undefined memory!") }
      yield memory[idx]
    }
  }

  const buffer = new Uint8Array(collect_str())
  const decoder = new TextDecoder("UTF-8");
  return decoder.decode(buffer);
}

function new_string(str) {
  const encoder = new TextEncoder("UTF-8");
  let buffer = encoder.encode(str);

  let len = buffer.length;
  let ptr = window.module.exports.alloc(len + 1);

  let memory = new Uint8Array(window.module.memory.buffer);
  for (let i = 0; i < len; i++) {
    memory[ptr + i] = buffer[i];
  }

  memory[ptr + len] = 0;

  return ptr;
}

function setup() {
  console.log("Setting up cedar!");

  var cedar = window.cedar = {};

  cedar.nodes = { "": document.body };

  let post = (msg) => window.module.exports.process(1234, new_string(msg));

  cedar.click = (id) => (event) => {
    var click = { "Click": { "id": id } };
    post(JSON.stringify(click));
  };

  cedar.input = (id, element) => (event) => {
    var value = element.value || '';
    var input = { Input: { id: id, value: value } };
    post(JSON.stringify(input));
  };

  cedar.keydown = (id, element) => (e) => {
    var input = { Keydown: { id: id, code: (e.keyCode ? e.keyCode : e.which) } };
    post(JSON.stringify(input));
  };

  cedar.attributes = (node, attributes) => {
    for (let attr in attributes) {
      var value = attributes[attr];

      // TODO: fix this!
      // HACK: convert to Boolean
      if (value == "true") { value = true; } else if (value == "false") { value = false; }

      node[attr] = value;
    }
  };

  cedar.command = (cmd) => {
    var cedar = window.cedar;

    var cmd = JSON.parse(cmd);

    if (cmd.hasOwnProperty('Create')) {
      var create = cmd.Create;

      var id = create.id;
      var parent = create.parent;

      var kind = create.kind;
      var value = create.value;

      var attributes = create.attributes;

      var node = kind == 'text' ? document.createTextNode(value) : document.createElement(kind);

      // TODO: only register for nodes that need to?
      // TODO: handle 'duplicate' events?

      node.addEventListener("click", cedar.click(id));

      node.addEventListener("input", cedar.input(id, node));
      node.addEventListener("keydown", cedar.keydown(id, node));

      // var input = cedar.input(id, node);
      // node.addEventListener("keypress", input);
      // node.addEventListener("input", input);
      // node.addEventListener("paste", input);

      cedar.attributes(node, attributes);

      var parent = cedar.nodes[parent];
      parent.appendChild(node);

      cedar.nodes[id] = node;
    } else if (cmd.hasOwnProperty('Update')) {
      var update = cmd.Update;

      var id = update.id;
      var value = update.value;

      var node = cedar.nodes[id];

      if (value.hasOwnProperty('Text')) {
        node.nodeValue = value.Text;
      } else if (value.hasOwnProperty('Attributes')) {
        cedar.attributes(node, value.Attributes);
      } else {
        // console.log("Unsupported value!");
      }
    } else if (cmd.hasOwnProperty('Remove')) {
      var remove = cmd.Remove;
      var id = remove.id;

      cedar.nodes[id].remove();
    }
  };
}

(function () {
  setup();

  window.module = {};

  const imports = {
    env: {
      alert: (ptr, len) => alert(copy_str(module, ptr, len)),
      log: (ptr, len) => console.log(copy_str(module, ptr, len)),

      command: (ptr, len) => {
        let s = copy_str(module, ptr, len);
        window.cedar.command(s);
      }
    }
  };

  fetch('code.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, imports))
    .then(({ module, instance }) => {
      window.module.memory = instance.exports.memory;
      window.module.exports = instance.exports;

      // console.log(instance.exports.add_one(41));
      // console.log(instance.exports.main());

      window.module.exports.main();
      // instance.exports.process(42);
    });
})();
