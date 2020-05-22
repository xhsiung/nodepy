def main( jstr ):
    import json
    obj = json.loads( jstr )
    return json.dumps(  {"success":True} )
