local ffi = require("ffi")
print("ffi->>", ffi)
-- for i, v in pairs(ffi) do 
--     print(i, v)
-- end
local c= ffi.cdef[[
   void add();
]]
print(c)


print("ffi end")

local c = ffi.load("E:\\private_work\\r_extern\\target\\debug\\r_extern")