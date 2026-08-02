const decoder = new TextDecoder("utf-8");

register_plugin = function (importObject) {
    importObject.env.js_console_log = function (ptr, len) {
        if (!wasm_memory) {
            wasm_memory = importObject.env.memory || wasm_exports.memory;
        }
        const buffer = new Uint8Array(wasm_memory.buffer, ptr, len);

        const text = decoder.decode(buffer);

        console.log(text);
    };
}

miniquad_add_plugin({
    register_plugin,
    version: "0.1.0",
    name: "additional_plugin"
})