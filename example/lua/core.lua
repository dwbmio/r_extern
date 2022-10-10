local ffi = require("ffi")
local c= ffi.cdef[[
   void add();
]]
print(c)


print("ffi end")

local c = ffi.load("E:\\private_work\\r_extern\\target\\i686-pc-windows-msvc\\release\\r_extern")
print(c)