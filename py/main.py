def main( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps(  {"success":True} )

def main2( jstr ):
    #add module py/mymodul.py
    import json
    import sys,os
    sys.path.append( "%s/py"%(os.getcwd()))
    
    return json.dumps(  {"success": False} )
