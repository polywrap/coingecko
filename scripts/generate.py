import os
import json
from typing import NamedTuple
from pprint import pprint
from textwrap import indent

from schemas import argSchema
from schemas import methodSchema
from schemas import moduleSchema
from schemas import libSchema
from schemas import funcSchema
from schemas import urlParamSchema
from schemas import methodBlueprintSchema


class Param(NamedTuple):
    name: str
    type: str = "string"


def pathParser(path: str):
    parts = path.split("/")
    params = set(filter(lambda x: len(x) > 1 and x[0] == "{" and x[-1] == "}", parts))
    titleName = "".join(
        map(
            lambda x: "".join(x.replace("_", " ").title().split(" ")),
            filter(lambda x: x not in params, parts),
        )
    )
    camelName = titleName[0].lower() + titleName[1:]

    return dict(map(lambda x: (x[1:-1], f"args.{x[1:-1]}"), params)), camelName


def translate(x: str, required: bool):
    translator = {"integer": "Int", "string": "String", "boolean": "Boolean"}

    return f"{translator[x]}!" if required else translator[x]


with open(os.path.join(os.path.dirname(__file__), "schema.json")) as f:
    schema = json.load(f)

    methods = []
    functions = []

    for path in schema["paths"]:

        params, name = pathParser(path)

        docstring = schema["paths"][path]["get"].get("summary", "")
        queryParams = schema["paths"][path]["get"].get("parameters", [])

        queryParams = list(
            map(
                lambda x: Param(x["name"], translate(x["type"], x["required"])),
                queryParams,
            )
        )

        args = indent(
            "\n".join(
                [argSchema.format(name=p.name, type=p.type) for p in queryParams]
            ),
            "  ",
            lambda _: True,
        )
        returnType = name[0].upper() + name[1:]
        method = methodSchema.format(
            name=name, docstring=docstring, args=args, returnType=returnType) if args else methodBlueprintSchema.format(name=name, docstring=docstring, returnType=returnType)

        urlParams = indent(",\n".join(urlParamSchema.format(key=param.name, value="args.name") for param in queryParams), " "*8, lambda _: True)
        argsName = "Args" + returnType
        
        func = funcSchema.format(name=name, args=argsName, path=path.format(**params), returnType=returnType, params=urlParams)
        
        functions.append(func)
        methods.append(method)

    methods = indent(
        "".join(methods),
        "  ",
        lambda _: True,
    )
    module = moduleSchema.format(methods=methods)

    functions = "\n\n\n".join(functions)
    lib = libSchema.format(functions=functions)


### GENERATE SCHEMA
with open(os.path.join(os.path.dirname(__file__), "..", "schema.graphql"), "w") as f:
    f.write(module)

### GENERATE RUST CODE
with open(os.path.join(os.path.dirname(__file__), "..", "src", "lib.rs"), "w") as f:
    f.write(lib)