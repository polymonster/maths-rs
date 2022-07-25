#!/opt/homebrew/bin/python3

from dataclasses import replace

import sys

def conv(str):
    str = str.replace("const vec3f", "let")
    str = str.replace("vec3f", "let")
    str = str.replace("(f32)", "")
    str = str.replace("f32", "let")
    str = str.replace("bool(1)", "true")
    str = str.replace("bool(0)", "false")
    str = str.replace("bool", "let")
    str = str.replace("u32(0)", "Classification::INTERSECTS")
    str = str.replace("u32(1)", "Classification::BEHIND")
    str = str.replace("u32(2)", "Classification::INFRONT")
    str = str.replace("u32", "let")
    str = str.replace("{", "vec3f(")
    str = str.replace("}", ")")
    str = str.replace("REQUIRE", "assert_eq!")
    str = str.replace("require_func", "")
    print(str)
    

if __name__ == "__main__":
    conv(sys.argv[1])