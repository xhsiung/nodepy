
def main( jstr ):
    import json
    import sys,os
    sys.path.append("%s/py"%os.getcwd())
    #print( sys.path )

    import main2
    main2.show()

    print( jstr )
    obj = json.loads( jstr )
    print( obj )
    
    return json.dumps( {"success":True} )