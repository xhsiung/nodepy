var addon = require('nodepy');
obj={"name":"alex","mobile":12345}
console.log(  addon.doSync("main" ,JSON.stringify( obj)) );

addon.doAsync("main2" ,JSON.stringify( obj), (err, val)=>{
	console.log("async");
	console.log( val );
});
