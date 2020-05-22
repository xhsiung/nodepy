# nodepy
Call python code from node.js.
## Usage

```javascript
var addon = require('nodepy');
obj={"name":"alex","mobile":12345}
console.log(  addon.doSync( JSON.stringify( obj)) );

addon.doAsync( JSON.stringify( obj), (err, value)=>{
        console.log("async");
        console.log( value );
});
```
## Feature

```py/main.py
def main( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps(  {"success":True} )

```
nodepy
