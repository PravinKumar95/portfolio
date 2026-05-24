const WebSocket = require('ws');

const wsUrl = process.argv[2];
const ws = new WebSocket(wsUrl);

ws.on('open', () => {
    // Enable DOM and CSS domains
    ws.send(JSON.stringify({ id: 1, method: 'DOM.enable' }));
    ws.send(JSON.stringify({ id: 2, method: 'CSS.enable' }));
    
    // Get the document root
    ws.send(JSON.stringify({ id: 3, method: 'DOM.getDocument', params: { depth: -1 } }));
});

ws.on('message', (data) => {
    const response = JSON.parse(data);
    
    if (response.id === 3) {
        // Find the H1 node
        const findH1 = (node) => {
            if (node.nodeName === 'H1') return node.nodeId;
            if (node.children) {
                for (const child of node.children) {
                    const found = findH1(child);
                    if (found) return found;
                }
            }
            return null;
        };
        const h1NodeId = findH1(response.result.root);
        if (h1NodeId) {
            // Get computed styles for the H1 node
            ws.send(JSON.stringify({ id: 4, method: 'CSS.getComputedStyleForNode', params: { nodeId: h1NodeId } }));
        } else {
            console.log('H1 not found');
            process.exit(1);
        }
    }
    
    if (response.id === 4) {
        console.log(JSON.stringify(response.result.computedStyle, null, 2));
        process.exit(0);
    }
});

ws.on('error', (err) => {
    console.error(err);
    process.exit(1);
});
