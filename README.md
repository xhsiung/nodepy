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
var addon = require('nodepy');
obj={"name":"alex","mobile":12345}
console.log(  addon.doSync("main" ,JSON.stringify( obj)) );

addon.doAsync("main2" ,JSON.stringify( obj), (err, val)=>{
        console.log("async");
        console.log( val );
});


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
