const os = require("os");
const {spawn} = require("child_process");

const qrcode = require("qrcode-terminal");
const termkit = require("terminal-kit");
const kill = require("tree-kill");
const term = termkit.terminal;

const copy_dir = require("./utils/copyDir");
const get_network_ip = require("./utils/getNetworkIP");


/**
 * TODO:
 *   - Figure out why colors have blaack backgrounds
 *   - Get loading spinners on titles while launching
 *   - Test on other platforms
 */


async function genQR() {
    return new Promise((resolve) => {
        qrcode.generate(`http://${get_network_ip()}:3000`, {small: true}, (qrcode) => {
            const h = qrcode.split("\n").length+1;
            const w = 2*(h-1);
            resolve({ w, h, qrcode });
        });
    });
}


// CLI
(async () => {
    term.clear();

    const spinner = await term.spinner();

    const document = term.createDocument();
    const QR = await genQR();

    spinner.destroy();

    term.hideCursor();

    // Initialize layout
    new termkit.Layout({
        parent: document,
        boxChars: "lightRounded",
        layout: {
            widthPercent: 100, heightPercent: 100,
            rows: [{
                heightPercent: 100,
                columns: [
                    {
                        rows: [{
                            id: "box1-row1",
                            height: 3,
                            columns: [{ id: "box1_1" }]
                        }, {
                            id: "box1-row2",
                            columns: [{ id: "box1_2" }]
                        }]
                    },
                    {
                        rows: [{
                            id: "box2-row1",
                            height: 3,
                            columns: [{ id: "box2_1" }]
                        }, {
                            id: "box2-row2",
                            columns: [{ id: "box2_2" }]
                        }]
                    },
                    { id: "qrbox", width: QR.w-1, percentHeight: 100 } ]
            }]
        }
    });

    // Box 1 (Frontend)
    new termkit.Text({
        parent: document.elements.box1_1,
        autoWidth: 1, autoHeight: 1,
        content: "Digital",
        leftPadding: " ",
        attr: { color: "blue" , bold: true }
    });
    const frontendBox = new termkit.TextBox({
        parent: document.elements.box1_2,
        scrollable: true, vScrollBar: true,
        contentHasMarkup: 'ansi',
        lineWrap: true,
        x: 0, y: 0,
        autoWidth: 1, autoHeight: 1
    });

    // Box 2 (Server)
    new termkit.Text({
        parent: document.elements.box2_1,
        autoWidth: 1, autoHeight: 1,
        content: "Server",
        leftPadding: " ",
        attr: { color: "blue" , bold: true }
    });
    const serverBox = new termkit.TextBox({
        parent: document.elements.box2_2,
        scrollable: true, vScrollBar: true,
        lineWrap: true,
        x: 0, y: 0,
        autoWidth: 1, autoHeight: 1
    });

    // QR Code window
    const window = new termkit.Window({
        parent: document.elements.qrbox,
        boxChars: "light",
        title: "^c^+Scan below^: to go to ^/site^:!",
        titleHasMarkup: true,
        movable: false,
        x: -1,
        width: QR.w, height: QR.h
    });
    const qrCodeBox = new termkit.TextBox({
        parent: window,
        lineWrap: false, wordWrap: false,
        scrollable: true,
        autoWidth: 1, autoHeight: 1
    });
    qrCodeBox.setContent(QR.qrcode);


    // Spawn digital server
    const digital = spawn("cd src/site/pages/digital && npm run start", {
        shell: true, env: { ...process.env, FORCE_COLOR: true }, detached: true
    });
    digital.stdout.on("data", (data) => frontendBox.appendLog(`${data}`));
    digital.stderr.on("data", (data) => frontendBox.appendLog(`${data}`));


    // Spawn backend server
    await buildServer(true);
    let server = start_server();

    function start_server() {
        const cmd = (os.platform() === "win32" ?
                        "cd build && server.exe -no_auth" :
                        "cd build && ./server -no_auth");
        const server = spawn(cmd, {
            shell: true, env: { ...process.env, FORCE_COLOR: true }, detached: true
        });
        server.stdout.on("data", (data) => serverBox.appendLog(`${data}`));
        server.stderr.on("data", (data) => serverBox.appendLog(`${data}`));
        return server;
    }
    function buildServer(firstTime = false) {
        return new Promise((resolve) => {
            serverBox.setContent("");

            copy_dir("src/server/data/sql/sqlite", "build/sql/sqlite");
            const cmd = (os.platform() === "win32" ?
                            "cd src/server && go build -o ../../build/server.exe" :
                            "cd src/server && go build -o ../../build/server");
            const child = spawn(cmd, {
                shell: true, env: { ...process.env, FORCE_COLOR: true }
            });
            child.stdout.on("data", (data) => serverBox.appendLog(`${data}`));
            child.stderr.on("data", (data) => serverBox.appendLog(`${data}`));

            child.on("exit", () => {
                if (!firstTime) {
                    // Restart server
                    server = start_server();
                }
                serverBox.setContent("");
                resolve();
            });
        });
    }


    // Exit safely on q, Q, or CTRL+C
    function exit() {
        term.grabInput(false);
        term.hideCursor(false);
        term.styleReset();
        term.clear();

        let killCount = 0;
        const onKill = () => {
            killCount++;
            if (killCount === 2)
                process.exit();
        }

        kill(digital.pid, onKill);
        kill(server.pid, onKill)
    }

    term.on("key", function(key) {
        if (key === "q" || key === "Q" || key === "CTRL_C")
            exit();
        if (key === "r" || key === "R") {
            // Kill currently running server
            kill(server.pid, buildServer);
        }
    });
    ["exit", "SIGINT", "SIGUSR1", "SIGUSR2", "uncaughtException", "SIGTERM"].forEach((ev) =>
        process.on(ev, exit)
    );
})();
