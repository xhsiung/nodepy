
def main( jstr ):
    import json
    import sys,os, time
    libpath = "%s/py"%os.getcwd()
    if libpath not in sys.path:
        sys.path.append(libpath)

    import main
    main.main()

    print( jstr )
    obj = json.loads( jstr )
    print( obj )
    
    return json.dumps( {"success":True} )