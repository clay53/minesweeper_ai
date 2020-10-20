const app = require('express')();

const home = require('./src/home')

app.get(
    [
        "/",
        "/index.html"
    ],
    home
);

app.get(
    [
        "/minesweeper_ai_wasm.js"
    ],
    (req, res) => {
        res.sendFile(__dirname + "/node_modules/minesweeper-ai-wasm/minesweeper_ai_wasm.js")
    }
)

app.get(
    [
        "/minesweeper_ai_wasm_bg.wasm"
    ],
    (req, res) => {
        res.sendFile(__dirname + "/node_modules/minesweeper-ai-wasm/minesweeper_ai_wasm_bg.wasm")
    }
)

app.get(
    [
        "/minesweeper_rs_wasm.js"
    ],
    (req, res) => {
        res.sendFile(__dirname + "/node_modules/minesweeper-rs-wasm/minesweeper_rs_wasm.js")
    }
)

app.get(
    [
        "/minesweeper_rs_wasm_bg.wasm"
    ],
    (req, res) => {
        res.sendFile(__dirname + "/node_modules/minesweeper-rs-wasm/minesweeper_rs_wasm_bg.wasm")
    }
)

app.listen(3000, () => {
    console.log("listening")
});