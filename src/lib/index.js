var addon = require('../native');
obj={"name":"alex","mobile":12345}
console.log(  addon.doSync( JSON.stringify( obj)) );

addon.doAsync( JSON.stringify( obj), (err, value)=>{
	console.log("async");
	console.log( value );
});
