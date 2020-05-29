def main( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps(  {"success":True} )

def main2( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps(  {"success":False} )
