extern crate libc_sys;
use libc_sys as libc;
pub static mut sqlite3UpperToLower: [libc::c_uchar; 256] =
    [0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar, 7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar, 9 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar, 11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar, 13 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar, 17 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar, 19 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar, 21 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar, 23 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar, 25 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar, 27 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar, 29 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar, 31 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar, 33 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar, 35 as libc::c_int as libc::c_uchar,
        36 as libc::c_int as libc::c_uchar, 37 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar, 39 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar, 41 as libc::c_int as libc::c_uchar,
        42 as libc::c_int as libc::c_uchar, 43 as libc::c_int as libc::c_uchar,
        44 as libc::c_int as libc::c_uchar, 45 as libc::c_int as libc::c_uchar,
        46 as libc::c_int as libc::c_uchar, 47 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar, 49 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar, 51 as libc::c_int as libc::c_uchar,
        52 as libc::c_int as libc::c_uchar, 53 as libc::c_int as libc::c_uchar,
        54 as libc::c_int as libc::c_uchar, 55 as libc::c_int as libc::c_uchar,
        56 as libc::c_int as libc::c_uchar, 57 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar, 59 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar, 61 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar, 63 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar, 97 as libc::c_int as libc::c_uchar,
        98 as libc::c_int as libc::c_uchar, 99 as libc::c_int as libc::c_uchar,
        100 as libc::c_int as libc::c_uchar, 101 as libc::c_int as libc::c_uchar,
        102 as libc::c_int as libc::c_uchar, 103 as libc::c_int as libc::c_uchar,
        104 as libc::c_int as libc::c_uchar, 105 as libc::c_int as libc::c_uchar,
        106 as libc::c_int as libc::c_uchar, 107 as libc::c_int as libc::c_uchar,
        108 as libc::c_int as libc::c_uchar, 109 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar, 111 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar, 113 as libc::c_int as libc::c_uchar,
        114 as libc::c_int as libc::c_uchar, 115 as libc::c_int as libc::c_uchar,
        116 as libc::c_int as libc::c_uchar, 117 as libc::c_int as libc::c_uchar,
        118 as libc::c_int as libc::c_uchar, 119 as libc::c_int as libc::c_uchar,
        120 as libc::c_int as libc::c_uchar, 121 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar, 91 as libc::c_int as libc::c_uchar,
        92 as libc::c_int as libc::c_uchar, 93 as libc::c_int as libc::c_uchar,
        94 as libc::c_int as libc::c_uchar, 95 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar, 97 as libc::c_int as libc::c_uchar,
        98 as libc::c_int as libc::c_uchar, 99 as libc::c_int as libc::c_uchar,
        100 as libc::c_int as libc::c_uchar, 101 as libc::c_int as libc::c_uchar,
        102 as libc::c_int as libc::c_uchar, 103 as libc::c_int as libc::c_uchar,
        104 as libc::c_int as libc::c_uchar, 105 as libc::c_int as libc::c_uchar,
        106 as libc::c_int as libc::c_uchar, 107 as libc::c_int as libc::c_uchar,
        108 as libc::c_int as libc::c_uchar, 109 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar, 111 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar, 113 as libc::c_int as libc::c_uchar,
        114 as libc::c_int as libc::c_uchar, 115 as libc::c_int as libc::c_uchar,
        116 as libc::c_int as libc::c_uchar, 117 as libc::c_int as libc::c_uchar,
        118 as libc::c_int as libc::c_uchar, 119 as libc::c_int as libc::c_uchar,
        120 as libc::c_int as libc::c_uchar, 121 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar, 123 as libc::c_int as libc::c_uchar,
        124 as libc::c_int as libc::c_uchar, 125 as libc::c_int as libc::c_uchar,
        126 as libc::c_int as libc::c_uchar, 127 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar, 129 as libc::c_int as libc::c_uchar,
        130 as libc::c_int as libc::c_uchar, 131 as libc::c_int as libc::c_uchar,
        132 as libc::c_int as libc::c_uchar, 133 as libc::c_int as libc::c_uchar,
        134 as libc::c_int as libc::c_uchar, 135 as libc::c_int as libc::c_uchar,
        136 as libc::c_int as libc::c_uchar, 137 as libc::c_int as libc::c_uchar,
        138 as libc::c_int as libc::c_uchar, 139 as libc::c_int as libc::c_uchar,
        140 as libc::c_int as libc::c_uchar, 141 as libc::c_int as libc::c_uchar,
        142 as libc::c_int as libc::c_uchar, 143 as libc::c_int as libc::c_uchar,
        144 as libc::c_int as libc::c_uchar, 145 as libc::c_int as libc::c_uchar,
        146 as libc::c_int as libc::c_uchar, 147 as libc::c_int as libc::c_uchar,
        148 as libc::c_int as libc::c_uchar, 149 as libc::c_int as libc::c_uchar,
        150 as libc::c_int as libc::c_uchar, 151 as libc::c_int as libc::c_uchar,
        152 as libc::c_int as libc::c_uchar, 153 as libc::c_int as libc::c_uchar,
        154 as libc::c_int as libc::c_uchar, 155 as libc::c_int as libc::c_uchar,
        156 as libc::c_int as libc::c_uchar, 157 as libc::c_int as libc::c_uchar,
        158 as libc::c_int as libc::c_uchar, 159 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar, 161 as libc::c_int as libc::c_uchar,
        162 as libc::c_int as libc::c_uchar, 163 as libc::c_int as libc::c_uchar,
        164 as libc::c_int as libc::c_uchar, 165 as libc::c_int as libc::c_uchar,
        166 as libc::c_int as libc::c_uchar, 167 as libc::c_int as libc::c_uchar,
        168 as libc::c_int as libc::c_uchar, 169 as libc::c_int as libc::c_uchar,
        170 as libc::c_int as libc::c_uchar, 171 as libc::c_int as libc::c_uchar,
        172 as libc::c_int as libc::c_uchar, 173 as libc::c_int as libc::c_uchar,
        174 as libc::c_int as libc::c_uchar, 175 as libc::c_int as libc::c_uchar,
        176 as libc::c_int as libc::c_uchar, 177 as libc::c_int as libc::c_uchar,
        178 as libc::c_int as libc::c_uchar, 179 as libc::c_int as libc::c_uchar,
        180 as libc::c_int as libc::c_uchar, 181 as libc::c_int as libc::c_uchar,
        182 as libc::c_int as libc::c_uchar, 183 as libc::c_int as libc::c_uchar,
        184 as libc::c_int as libc::c_uchar, 185 as libc::c_int as libc::c_uchar,
        186 as libc::c_int as libc::c_uchar, 187 as libc::c_int as libc::c_uchar,
        188 as libc::c_int as libc::c_uchar, 189 as libc::c_int as libc::c_uchar,
        190 as libc::c_int as libc::c_uchar, 191 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar, 193 as libc::c_int as libc::c_uchar,
        194 as libc::c_int as libc::c_uchar, 195 as libc::c_int as libc::c_uchar,
        196 as libc::c_int as libc::c_uchar, 197 as libc::c_int as libc::c_uchar,
        198 as libc::c_int as libc::c_uchar, 199 as libc::c_int as libc::c_uchar,
        200 as libc::c_int as libc::c_uchar, 201 as libc::c_int as libc::c_uchar,
        202 as libc::c_int as libc::c_uchar, 203 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar, 205 as libc::c_int as libc::c_uchar,
        206 as libc::c_int as libc::c_uchar, 207 as libc::c_int as libc::c_uchar,
        208 as libc::c_int as libc::c_uchar, 209 as libc::c_int as libc::c_uchar,
        210 as libc::c_int as libc::c_uchar, 211 as libc::c_int as libc::c_uchar,
        212 as libc::c_int as libc::c_uchar, 213 as libc::c_int as libc::c_uchar,
        214 as libc::c_int as libc::c_uchar, 215 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar, 217 as libc::c_int as libc::c_uchar,
        218 as libc::c_int as libc::c_uchar, 219 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar, 221 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar, 223 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar, 225 as libc::c_int as libc::c_uchar,
        226 as libc::c_int as libc::c_uchar, 227 as libc::c_int as libc::c_uchar,
        228 as libc::c_int as libc::c_uchar, 229 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar, 231 as libc::c_int as libc::c_uchar,
        232 as libc::c_int as libc::c_uchar, 233 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar, 235 as libc::c_int as libc::c_uchar,
        236 as libc::c_int as libc::c_uchar, 237 as libc::c_int as libc::c_uchar,
        238 as libc::c_int as libc::c_uchar, 239 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar, 241 as libc::c_int as libc::c_uchar,
        242 as libc::c_int as libc::c_uchar, 243 as libc::c_int as libc::c_uchar,
        244 as libc::c_int as libc::c_uchar, 245 as libc::c_int as libc::c_uchar,
        246 as libc::c_int as libc::c_uchar, 247 as libc::c_int as libc::c_uchar,
        248 as libc::c_int as libc::c_uchar, 249 as libc::c_int as libc::c_uchar,
        250 as libc::c_int as libc::c_uchar, 251 as libc::c_int as libc::c_uchar,
        252 as libc::c_int as libc::c_uchar, 253 as libc::c_int as libc::c_uchar,
        254 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar];