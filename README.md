# nodepy
Call python code from node.js.

## Structure
![image](https://raw.githubusercontent.com/xhsiung/nodepy/master/imgs/struct.png)
```
├── node_modules
│   └── nodepy
│       └── index.node  //addon module
├── py
│   └── main.py  //編輯此檔
└── test.js
```

## JS Usage
```
var addon = require('nodepy');
//sync
addon.doSync(fnstr,str);

//async
addon.doAsync(fnstr,str,(err,val)=>{});
```

## Python Usage
```
#py/main.py
def main(str):
	//do something here
	def run():
		print("run)
	def run2():
		run()
	run()
	return str
```

## Exmaple
```javascript
//test.js
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

//py/main.py
def main( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps( {"success":True} )

def main2( jstr ):
    import json
    import sys,os
    sys.path.append("%s/py"%os.getcwd())
    #add module py/mymodule.py
    import mymodule
    return json.dumps( {"success": False} )
```

nodepy
