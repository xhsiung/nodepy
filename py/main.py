
def main( jstr ):
    import json
    import sys,os, time
    libpath = "%s/py"%os.getcwd()
    if libpath not in sys.path:
        sys.path.append(libpath)

    import main2
    main2.show()

    print( jstr )
    obj = json.loads( jstr )
    print( obj )
    
    return json.dumps( {"success":True} )