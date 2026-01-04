// #region agent log
// Port conflict detection script
import http from 'http';
import fs from 'fs';
import path from 'path';

const logPath = '/home/swamizero/.cursor/debug.log';

const log = (data) => {
  try {
    const logEntry = JSON.stringify({
      ...data,
      timestamp: Date.now(),
      sessionId: 'debug-session',
      runId: 'port-check'
    }) + '\n';
    fs.appendFileSync(logPath, logEntry, 'utf8');
  } catch (e) {
    // Ignore log errors
  }
};
// #endregion

const checkPort = (port) => {
  return new Promise((resolve) => {
    const server = http.createServer();
    server.listen(port, '127.0.0.1', () => {
      server.once('close', () => {
        // #region agent log
        log({
          location: 'check-port.js:checkPort',
          message: 'Port available',
          data: { port },
          hypothesisId: 'A'
        });
        // #endregion
        resolve(true);
      });
      server.close();
    });
    server.on('error', (err) => {
      if (err.code === 'EADDRINUSE') {
        // #region agent log
        log({
          location: 'check-port.js:checkPort',
          message: 'Port conflict detected',
          data: { port, error: err.code },
          hypothesisId: 'A'
        });
        // #endregion
        resolve(false);
      } else {
        resolve(true);
      }
    });
  });
};

const main = async () => {
  const port = parseInt(process.argv[2] || '1420', 10);
  const available = await checkPort(port);
  
  // #region agent log
  log({
    location: 'check-port.js:main',
    message: 'Port check completed',
    data: { port, available },
    hypothesisId: 'A'
  });
  // #endregion
  
  if (!available) {
    console.error(`❌ Port ${port} is already in use!`);
    console.error('Run: npm run cleanup');
    process.exit(1);
  } else {
    console.log(`✅ Port ${port} is available`);
    process.exit(0);
  }
};

main();
// #endregion


