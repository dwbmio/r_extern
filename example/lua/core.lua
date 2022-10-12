local ffi = require("ffi")
local c = ffi.cdef [[
   //test
   int add(int a);
   //http
   void http_set_host(const char* b);
   void http_set_timeout(int a);
   const char* http_get(const char* b);
]]
print(debug.getinfo())
package.cpath = package.cpath .. ";..\\..\\..\\target\\i686-pc-windows-msvc\\release\\"
print(package.cpath)
ffi.load("r_extern")

print(package.path)
-- package.cpath = p
local c = ffi.load("D:\\private_work\\r_extern\\target\\i686-pc-windows-msvc\\release\\r_extern")

local function test()
    assert(c.add(2) == 4, "c.add failed!")
end

local function test_http()
    print(c.http_set_host("http://static.bbclient.icu:8083"))
    print(c.http_set_timeout(5))
    -- todo assert 
    print(ffi.string(c.http_get("api/dingcode/dnode")))
end

test()
-- test_http()
