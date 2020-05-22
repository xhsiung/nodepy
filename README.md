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

## Usage
```javascript
var addon = require('nodepy');
obj={"name":"alex","mobile":12345}

//sync
console.log(  addon.doSync( JSON.stringify( obj)) );

//async
addon.doAsync( JSON.stringify( obj), (err, value)=>{
        console.log("async");
        console.log( value );
});
```

## Feature
```py/main.py
#py/main.py
def main( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps(  {"success":True} )

```
nodepy
