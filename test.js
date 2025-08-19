const { doSync,doAsyncTask, doAsyncTaskCallback } = require('./nodepy');

async function main() {
  const fnstr = 'main';
  const jstr = JSON.stringify({ message: 'Hello from Node.js!' });

  //Test doSync
  console.log('\nTesting doSync...');
  const result = doSync(fnstr, jstr);
  console.log('doSync result:', result);

  // Test doAsyncTask
  console.log('\nTesting doAsyncTask...');
  try {
    const result = await doAsyncTask(fnstr, jstr);
    console.log('doAsyncTask result:', result);
  } catch (err) {
    console.error('doAsyncTask error:', err);
  }
  // Test doAsyncTaskCallback
  console.log('\nTesting doAsyncTaskCallback...');
  doAsyncTaskCallback(fnstr, jstr, (r1,r2) => {
    console.log('doAsyncTaskCallback result:', r1, r2);
    console.log( r2);
  });

}

main().catch(console.error);
