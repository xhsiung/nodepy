def main( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps(  {"success":True} )

def main2( jstr ):
    import json
    import sys,os
    sys.path.append( "%s/py"%os.getcwd())
    #add module py/mymodule.py
    import mymodule
    return json.dumps(  {"success": False} )
