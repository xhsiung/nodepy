const { doSync, doAsyncTask, doAsyncTaskCallback } = require('./nodepy');

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
  try {
    const result = await doAsyncTaskCallback(fnstr, jstr, (err, result) => {
      if (err) {
        console.error('doAsyncTaskCallback (callback) error:', err);
        return;
      }
      console.log('doAsyncTaskCallback (callback) result:', result);
    });
    console.log('doAsyncTaskCallback (promise) result:', result);
  } catch (err) {
    console.error('doAsyncTaskCallback (promise) error:', err);
  }

}

main().catch(console.error);
