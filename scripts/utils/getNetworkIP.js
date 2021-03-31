const { networkInterfaces } = require("os");

// From https://stackoverflow.com/questions/3653065/get-local-ip-address-in-node-js
module.exports = function get_network_ip() {
    const nets = networkInterfaces();
    const results = {};

    for (const name of Object.keys(nets)) {
        for (const net of nets[name]) {
            // Skip over non-IPv4 and internal (i.e. 127.0.0.1) addresses
            if (net.family === "IPv4" && !net.internal) {
                if (!results[name])
                    results[name] = [];
                results[name].push(net.address);
            }
        }
    }

    return results["en0"][0];
}
