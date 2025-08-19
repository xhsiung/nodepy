const { doSync, doAsync, doSyncCallback, doAsyncCallback } = require('./nodepy');

async function main() {
  const fnstr = 'main';
  const jstr = JSON.stringify({ message: 'Hello from Node.js!' });

  // Test doSync
  console.log('Testing doSync...');
  const syncResult = doSync(fnstr, jstr);
  console.log('doSync result:', syncResult);

  // Test doAsync
  console.log('\nTesting doAsync...');
  const asyncResult = await doAsync(fnstr, jstr);
  console.log('doAsync result:', asyncResult);

  // Test doSyncCallback
  console.log('\nTesting doSyncCallback...');
  doSyncCallback(fnstr, jstr, (err, result) => {
    if (err) {
      console.error('doSyncCallback error:', err);
      return;
    }
    console.log('doSyncCallback result:', result);
  });

  // Test doAsyncCallback
  console.log('\nTesting doAsyncCallback...');
  await new Promise((resolve) => {
    doAsyncCallback(fnstr, jstr, (err, result) => {
      if (err) {
        console.error('doAsyncCallback error:', err);
      } else {
        console.log('doAsyncCallback result:', result);
      }
      resolve();
    });
  });
}

main().catch(console.error);
