const fs = require('fs');
const path = require('path');

const targetPaths = [
  path.join(__dirname, '../graphify-out/graph.html'),
  path.join(__dirname, '../docs/graph.html')
];

const injectedScript = `
        <style>
            #sidebar { display: none !important; }
            #graph   { width: 100% !important; flex: 1 !important; }
            body     { overflow: hidden !important; }
        </style>
        <script>
            // Expose on window so parent iframe can access
            window.network = network;

            network.once('stabilizationIterationsDone', () => {
                network.setOptions({ physics: { enabled: false } });
                window.networkReady = true;

                // ── Cinematic node-touring animation ─────────────────────────────────────
                const hubNodes = RAW_NODES.slice().sort((a, b) => b.degree - a.degree).slice(0, 40);

                let tourStep = 0;
                let tourTimer = null;
                let overviewCounter = 0;

                function rand(a, b) { return a + Math.random() * (b - a); }

                function runTour() {
                    if (window.phantomPaused) {
                        tourTimer = setTimeout(runTour, 500);
                        return;
                    }

                    const doOverview = (overviewCounter % 4 === 0);
                    overviewCounter++;

                    if (doOverview) {
                        network.unselectAll();
                        network.moveTo({
                            position: { x: rand(-80, 80), y: rand(-60, 60) },
                            scale: rand(0.22, 0.32),
                            animation: { duration: Math.round(rand(3000, 4200)), easingFunction: 'easeInOutCubic' }
                        });
                        tourTimer = setTimeout(runTour, rand(4200, 5500));
                        return;
                    }

                    const node = hubNodes[tourStep % hubNodes.length];
                    tourStep++;

                    const positions = network.getPositions([node.id]);
                    const pos = positions[node.id];
                    if (!pos) { tourTimer = setTimeout(runTour, 400); return; }

                    network.unselectAll();
                    network.selectNodes([node.id]);

                    const nodeScale = rand(1.4, 2.4);
                    const travelTime = Math.round(rand(2800, 4000));

                    network.moveTo({
                        position: { x: pos.x, y: pos.y },
                        scale: nodeScale,
                        animation: { duration: travelTime, easingFunction: 'easeInOutCubic' }
                    });

                    const holdTime = rand(1000, 2000);
                    tourTimer = setTimeout(runTour, travelTime + holdTime);
                }

                tourTimer = setTimeout(runTour, 1200);

                window.stopTour = () => { clearTimeout(tourTimer); network.unselectAll(); };
                window.startTour = () => { tourTimer = setTimeout(runTour, 600); };
            });
        </script>
`;

for (const file of targetPaths) {
  if (fs.existsSync(file)) {
    let html = fs.readFileSync(file, 'utf8');
    if (!html.includes('id="sidebar" { display: none')) {
      html = html.replace('</body>', injectedScript + '\n</body>');
      fs.writeFileSync(file, html);
      console.log('Successfully injected animation and styles into ' + file);
    } else {
      console.log(file + ' already injected.');
    }
  }
}
