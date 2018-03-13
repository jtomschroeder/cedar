"use strict";

// Adapted from https://github.com/badboy/hellorust/blob/master/demos/bundle.js

class CString {
    static copy(module, ptr, len) {
        const collector = function* () {
            let memory = new Uint8Array(module.memory.buffer);
            for (let i = 0; i < len; ++i) {
                let idx = ptr + i;
                if (memory[idx] === undefined) {
                    throw new Error("Attempted to read undefined memory!")
                }
                yield memory[idx]
            }
        };

        const buffer = new Uint8Array(collector())
        const decoder = new TextDecoder("UTF-8");

        return decoder.decode(buffer);
    }

    static from(str) {
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
}

function setup() {
    console.log("Setting up cedar!");

    let cedar = window.cedar = {};

    cedar.nodes = { "": document.body };

    let post = window.post = (msg) => {
        const m = JSON.stringify(msg);
        window.module.exports.process(CString.from(m));
    };

    cedar.click = (id) => (event) => {
        post({ "Click": { "id": id } });
    };

    cedar.input = (id, element) => (event) => {
        var value = element.value || '';
        post({ Input: { id: id, value: value } });
    };

    cedar.keydown = (id, element) => (e) => {
        post({ Keydown: { id: id, code: (e.keyCode ? e.keyCode : e.which) } });
    };

    cedar.attributes = (node, attributes) => {
        for (const attr in attributes) {
            var value = attributes[attr];

            // TODO: fix this!
            // HACK: convert to Boolean
            if (value === "true") {
                value = true;
            } else if (value === "false") {
                value = false;
            } else {
                // value is untouched
            }

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

            var node = kind === 'text' ? document.createTextNode(value) : document.createElement(kind);

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
            alert: (ptr, len) => alert(CString.copy(module, ptr, len)),
            log: (ptr, len) => console.log(CString.copy(module, ptr, len)),

            command: (ptr, len) => {
                let s = CString.copy(module, ptr, len);
                window.cedar.command(s);
            },

            execute: (ptr, len) => {
                let code = CString.copy(module, ptr, len);
                Function(`"use strict"; return ${code}`)();
            }
        }
    };

    fetch('code.wasm')
        .then(response => response.arrayBuffer())
        .then(bytes => WebAssembly.instantiate(bytes, imports))
        .then(({ module, instance }) => {
            window.module.memory = instance.exports.memory;
            window.module.exports = instance.exports;

            window.module.exports.main();
        });
})();
