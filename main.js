import init, { run_app } from '/simon-at-simon/pkg/yew_wasm_pack_minimal.js';
async function main() {
   await init('/simon-at-simon/pkg/yew_wasm_pack_minimal_bg.wasm');
   run_app();
}
main()
