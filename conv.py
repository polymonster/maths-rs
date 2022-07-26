from dataclasses import replace

import sys

# converts c++ test code to rust, with some caveats
def conv(str):
    str = str.replace("const vec3f", "let")
    str = str.replace("const mat4", "let")
    str = str.replace("vec3f", "let")
    str = str.replace("(f32)", "")
    str = str.replace("f32", "let")
    str = str.replace("bool(1)", "true")
    str = str.replace("bool(0)", "false")
    str = str.replace("bool", "let")
    str = str.replace("u32(0)", "Classification::Intersects")
    str = str.replace("u32(1)", "Classification::Behind")
    str = str.replace("u32(2)", "Classification::Infront")
    str = str.replace("u32", "let")
    str = str.replace("{", "vec3f(")
    str = str.replace("}", ")")
    str = str.replace("REQUIRE", "assert_eq!")
    str = str.replace("require_func", "")
    str = str.replace("((", "(")
    str = str.replace("))", ")")
    print(str)
    

if __name__ == "__main__":
    conv(sys.argv[1])