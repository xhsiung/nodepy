# nodepy
Call python code from node.js.

## Structure
![image](https://raw.githubusercontent.com/xhsiung/nodepy/master/imgs/struct.png)
```
├── node_modules
│   └── nodepy
│       └── nodepy.node  //addon module
├── py
│   └── __init__.py  //程式進入def main():
│   └── main.py      //編輯此檔
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

//test.js
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
```

nodepy
