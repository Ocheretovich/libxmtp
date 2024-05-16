// import { WasmXmtpClient } from "./dist/node/bindings_wasm.js";
import { HelloWorld } from "./dist/node/bindings_wasm.js";

async function run() {
  try {
    let client = new HelloWorld("http://localhost:5555");
    console.log("Client created successfully", client);
  } catch (e) {
    console.error("Failed to create client", e);
  }
}

run();
