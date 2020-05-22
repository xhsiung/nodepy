var addon = require('nodepy');
obj={"name":"alex","mobile":12345}
console.log(  addon.doSync( JSON.stringify( obj)) );

addon.doAsync( JSON.stringify( obj), (err, val)=>{
	console.log("async");
	console.log( val );
});
