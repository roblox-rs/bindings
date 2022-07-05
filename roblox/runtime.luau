local rt = (function()
	local I64 = (function()
		local Numeric = {}

		local BIT_SET_31 = 0x80000000
		local BIT_SET_32 = 0x100000000

		local K_ZERO, K_ONE, K_BIT_SET_26

		local bit_lshift = bit32.lshift
		local bit_rshift = bit32.rshift
		local bit_arshift = bit32.arshift

		local bit_and = bit32.band
		local bit_or = bit32.bor
		local bit_xor = bit32.bxor
		local bit_not = bit32.bnot

		local bit_replace = bit32.replace

		local math_ceil = math.ceil
		local math_floor = math.floor
		local math_log = math.log
		local math_max = math.max
		local math_pow = math.pow

		local table_freeze = table.freeze

		local from_u32, into_u32, from_u64, into_u64
		local num_add, num_subtract, num_multiply, num_divide_unsigned, num_negate, num_not
		local num_is_negative, num_is_zero, num_is_equal, num_is_less_unsigned, num_is_greater_unsigned

		-- TODO: Eventually support Vector3
		function Numeric.from_u32(data_1, data_2)
			return table_freeze({ data_1, data_2 })
		end

		function Numeric.into_u32(data)
			return data[1], data[2]
		end

		function Numeric.from_u64(value)
			return from_u32(bit_and(value), math_floor(value / BIT_SET_32))
		end

		function Numeric.into_u64(value)
			local data_1, data_2 = into_u32(value)

			return data_1 + data_2 * BIT_SET_32
		end

		function Numeric.add(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			local data_1 = data_l_1 + data_r_1
			local data_2 = data_l_2 + data_r_2

			if data_1 >= BIT_SET_32 then
				data_1 = data_1 - BIT_SET_32
				data_2 = data_2 + 1
			end

			if data_2 >= BIT_SET_32 then
				data_2 = data_2 - BIT_SET_32
			end

			return from_u32(data_1, data_2)
		end

		function Numeric.subtract(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			local data_1 = data_l_1 - data_r_1
			local data_2 = data_l_2 - data_r_2

			if data_1 < 0 then
				data_1 = data_1 + BIT_SET_32
				data_2 = data_2 - 1
			end

			if data_2 < 0 then
				data_2 = data_2 + BIT_SET_32
			end

			return from_u32(data_1, data_2)
		end

		local function set_absolute(lhs, rhs)
			local has_negative = false

			if num_is_negative(lhs) then
				lhs = num_negate(lhs)
				has_negative = not has_negative
			end

			if num_is_negative(rhs) then
				rhs = num_negate(rhs)
				has_negative = not has_negative
			end

			return has_negative, lhs, rhs
		end

		function Numeric.multiply(lhs, rhs)
			if num_is_zero(lhs) or num_is_zero(rhs) then
				return K_ZERO
			end

			local has_negative

			has_negative, lhs, rhs = set_absolute(lhs, rhs)

			-- If both longs are small, use float multiplication
			if num_is_less_unsigned(lhs, K_BIT_SET_26) and num_is_less_unsigned(rhs, K_BIT_SET_26) then
				local data_l_1, _ = into_u32(lhs)
				local data_r_1, _ = into_u32(rhs)
				local result = from_u64(data_l_1 * data_r_1)

				if has_negative then
					result = num_negate(result)
				end

				return result
			end

			-- Divide each long into 4 chunks of 16 bits, and then add up 4x4 products.
			-- We can skip products that would overflow.
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			local a48 = bit_rshift(data_l_2, 16)
			local a32 = bit_and(data_l_2, 0xFFFF)
			local a16 = bit_rshift(data_l_1, 16)
			local a00 = bit_and(data_l_1, 0xFFFF)

			local b48 = bit_rshift(data_r_2, 16)
			local b32 = bit_and(data_r_2, 0xFFFF)
			local b16 = bit_rshift(data_r_1, 16)
			local b00 = bit_and(data_r_1, 0xFFFF)

			local c00 = a00 * b00
			local c16 = bit_rshift(c00, 16)

			c00 = bit_and(c00, 0xFFFF)
			c16 = c16 + a16 * b00

			local c32 = bit_rshift(c16, 16)

			c16 = bit_and(c16, 0xFFFF)
			c16 = c16 + a00 * b16
			c32 = c32 + bit_rshift(c16, 16)
			c16 = bit_and(c16, 0xFFFF)
			c32 = c32 + a32 * b00

			local c48 = bit_rshift(c32, 16)

			c32 = bit_and(c32, 0xFFFF)
			c32 = c32 + a16 * b16
			c48 = c48 + bit_rshift(c32, 16)
			c32 = bit_and(c32, 0xFFFF)
			c32 = c32 + a00 * b32
			c48 = c48 + bit_rshift(c32, 16)
			c32 = bit_and(c32, 0xFFFF)
			c48 = c48 + a48 * b00 + a32 * b16 + a16 * b32 + a00 * b48
			c48 = bit_and(c48, 0xFFFF)

			local data_1 = bit_replace(c00, c16, 16, 16)
			local data_2 = bit_replace(c32, c48, 16, 16)
			local result = from_u32(data_1, data_2)

			if has_negative then
				result = num_negate(result)
			end

			return result
		end

		local function get_approx_delta(rem, rhs)
			local approx = math_max(1, math_floor(rem / rhs))
			local log = math_ceil(math_log(approx, 2))
			local delta = log <= 48 and 1 or math_pow(2, log - 48)

			return approx, delta
		end

		function Numeric.divide_unsigned(lhs, rhs)
			if num_is_zero(rhs) then
				error("division by zero")
			elseif num_is_zero(lhs) then
				return K_ZERO
			end

			local rhs_number = into_u64(rhs)
			local rem = lhs
			local res = K_ZERO

			while num_is_greater_unsigned(rem, rhs) or num_is_equal(rem, rhs) do
				local res_approx, delta = get_approx_delta(into_u64(rem), rhs_number)
				local res_temp = from_u64(res_approx)
				local rem_temp = num_multiply(res_temp, rhs)

				while num_is_negative(rem_temp) or num_is_greater_unsigned(rem_temp, rem) do
					res_approx = res_approx - delta
					res_temp = from_u64(res_approx)
					rem_temp = num_multiply(res_temp, rhs)
				end

				if num_is_zero(res_temp) then
					res_temp = K_ONE
				end

				res = num_add(res, res_temp)
				rem = num_subtract(rem, rem_temp)
			end

			return res
		end

		function Numeric.divide_signed(lhs, rhs)
			local has_negative

			has_negative, lhs, rhs = set_absolute(lhs, rhs)

			local result = num_divide_unsigned(lhs, rhs)

			if has_negative then
				result = num_negate(result)
			end

			return result
		end

		function Numeric.negate(value)
			return num_add(num_not(value), K_ONE)
		end

		function Numeric.bit_and(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			return from_u32(bit_and(data_l_1, data_r_1), bit_and(data_l_2, data_r_2))
		end

		function Numeric.bit_not(value)
			local data_1, data_2 = into_u32(value)

			return from_u32(bit_not(data_1), bit_not(data_2))
		end

		function Numeric.bit_or(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			return from_u32(bit_or(data_l_1, data_r_1), bit_or(data_l_2, data_r_2))
		end

		function Numeric.bit_xor(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			return from_u32(bit_xor(data_l_1, data_r_1), bit_xor(data_l_2, data_r_2))
		end

		function Numeric.shift_left(lhs, rhs)
			local count = into_u64(rhs)

			if count < 32 then
				local pad = 32 - count
				local data_l_1, data_l_2 = into_u32(lhs)

				local data_1 = bit_lshift(data_l_1, count)
				local data_2 = bit_replace(bit_rshift(data_l_1, pad), data_l_2, count, pad)

				return from_u32(data_1, data_2)
			elseif count == 32 then
				local data_l_1, _ = into_u32(lhs)

				return from_u32(0, data_l_1)
			else
				local data_l_1, _ = into_u32(lhs)

				return from_u32(0, bit_lshift(data_l_1, count - 32))
			end
		end

		function Numeric.shift_right_unsigned(lhs, rhs)
			local count = into_u64(rhs)

			if count < 32 then
				local data_l_1, data_l_2 = into_u32(lhs)

				local data_1 = bit_replace(bit_rshift(data_l_1, count), data_l_2, 32 - count, count)
				local data_2 = bit_rshift(data_l_2, count)

				return from_u32(data_1, data_2)
			elseif count == 32 then
				local _, data_l_2 = into_u32(lhs)

				return from_u32(data_l_2, 0)
			else
				local _, data_l_2 = into_u32(lhs)

				return from_u32(bit_rshift(data_l_2, count - 32), 0)
			end
		end

		function Numeric.shift_right_signed(lhs, rhs)
			local count = into_u64(rhs)

			if count < 32 then
				local data_l_1, data_l_2 = into_u32(lhs)

				local data_1 = bit_replace(bit_rshift(data_l_1, count), data_l_2, 32 - count, count)
				local data_2 = bit_arshift(data_l_2, count)

				return from_u32(data_1, data_2)
			else
				local _, data_l_2 = into_u32(lhs)

				local data_1 = bit_arshift(data_l_2, count - 32)
				local data_2 = data_l_2 > BIT_SET_31 and BIT_SET_32 - 1 or 0

				return from_u32(data_1, data_2)
			end
		end

		function Numeric.is_negative(value)
			local _, data_2 = into_u32(value)

			return data_2 >= BIT_SET_31
		end

		function Numeric.is_zero(value)
			local data_1, data_2 = into_u32(value)

			return data_1 == 0 and data_2 == 0
		end

		function Numeric.is_equal(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			return data_l_1 == data_r_1 and data_l_2 == data_r_2
		end

		function Numeric.is_less_unsigned(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			return data_l_2 < data_r_2 or (data_l_2 == data_r_2 and data_l_1 < data_r_1)
		end

		function Numeric.is_greater_unsigned(lhs, rhs)
			local data_l_1, data_l_2 = into_u32(lhs)
			local data_r_1, data_r_2 = into_u32(rhs)

			return data_l_2 > data_r_2 or (data_l_2 == data_r_2 and data_l_1 > data_r_1)
		end

		function Numeric.is_less_signed(lhs, rhs)
			local neg_a = num_is_negative(lhs)
			local neg_b = num_is_negative(rhs)

			if neg_a and not neg_b then
				return true
			elseif not neg_a and neg_b then
				return false
			else
				return num_is_negative(num_subtract(lhs, rhs))
			end
		end

		function Numeric.is_greater_signed(lhs, rhs)
			local neg_a = num_is_negative(lhs)
			local neg_b = num_is_negative(rhs)

			if neg_a and not neg_b then
				return false
			elseif not neg_a and neg_b then
				return true
			else
				return num_is_negative(num_subtract(rhs, lhs))
			end
		end

		from_u32 = Numeric.from_u32
		into_u32 = Numeric.into_u32
		from_u64 = Numeric.from_u64
		into_u64 = Numeric.into_u64

		num_add = Numeric.add
		num_subtract = Numeric.subtract
		num_multiply = Numeric.multiply
		num_divide_unsigned = Numeric.divide_unsigned
		num_negate = Numeric.negate
		num_not = Numeric.bit_not

		num_is_negative = Numeric.is_negative
		num_is_zero = Numeric.is_zero
		num_is_equal = Numeric.is_equal
		num_is_less_unsigned = Numeric.is_less_unsigned
		num_is_greater_unsigned = Numeric.is_greater_unsigned

		K_ZERO = from_u64(0)
		K_ONE = from_u64(1)
		K_BIT_SET_26 = from_u64(0x4000000)

		Numeric.K_ZERO = K_ZERO
		Numeric.K_ONE = K_ONE

		return table_freeze(Numeric)
	end)()
	local module = {}

	local MAX_UNSIGNED = 0xffffffff
	local MAX_SIGNED = 0x7fffffff
	local BIT_SET_27 = 0x8000000
	local BIT_SET_32 = 0x100000000

	local to_u32 = bit32.band

	local bit_or = bit32.bor
	local bit_and = bit32.band
	local bit_lshift = bit32.lshift
	local bit_rshift = bit32.rshift

	local num_from_u32 = I64.from_u32
	local num_into_u32 = I64.into_u32

	local function to_i32(num)
		if num > MAX_SIGNED then
			num = num - BIT_SET_32
		end

		return num
	end

	local function no_op(num)
		return num
	end

	do
		local temp = {}

		temp.K_ZERO = I64.K_ZERO
		temp.K_ONE = I64.K_ONE

		temp.from_u32 = num_from_u32

		module.i64 = temp
	end

	do
		local add = {}
		local sub = {}
		local mul = {}
		local div = {}
		local rem = {}
		local neg = {}
		local min = {}
		local max = {}
		local copysign = {}
		local nearest = {}

		local assert = assert

		local math_abs = math.abs
		local math_fmod = math.fmod
		local math_floor = math.floor
		local math_round = math.round
		local math_sign = math.sign
		local math_max = math.max
		local math_min = math.min

		local string_byte = string.byte
		local string_pack = string.pack

		function add.i32(a, b)
			return to_u32(a + b)
		end

		function sub.i32(a, b)
			return to_u32(a - b)
		end

		function mul.i32(a, b)
			if (a + b) < BIT_SET_27 then
				return to_u32(a * b)
			else
				local a16 = bit_rshift(a, 16)
				local a00 = bit_and(a, 0xFFFF)
				local b16 = bit_rshift(b, 16)
				local b00 = bit_and(b, 0xFFFF)

				local c00 = a00 * b00
				local c16 = a16 * b00 + a00 * b16

				return to_u32(c00 + bit_lshift(c16, 16))
			end
		end

		function div.i32(lhs, rhs)
			assert(rhs ~= 0, "division by zero")

			lhs = to_i32(lhs)
			rhs = to_i32(rhs)

			return to_u32(lhs / rhs)
		end

		function div.u32(lhs, rhs)
			assert(rhs ~= 0, "division by zero")

			return to_u32(lhs / rhs)
		end

		function rem.i32(lhs, rhs)
			assert(rhs ~= 0, "division by zero")

			lhs = to_i32(lhs)
			rhs = to_i32(rhs)

			return to_u32(math_fmod(lhs, rhs))
		end

		add.i64 = I64.add
		sub.i64 = I64.subtract
		mul.i64 = I64.multiply
		div.i64 = I64.divide_signed
		div.u64 = I64.divide_unsigned

		function neg.f32(num)
			return -num
		end

		function min.f32(a, b)
			if b == b then
				return math_min(a, b)
			else
				return b
			end
		end

		function max.f32(a, b)
			if b == b then
				return math_max(a, b)
			else
				return b
			end
		end

		function copysign.f32(lhs, rhs)
			local packed = string_pack("<d", rhs)
			local sign = string_byte(packed, 8)

			if sign < 0x80 then
				return math_abs(lhs)
			else
				return -math_abs(lhs)
			end
		end

		function nearest.f32(num)
			local result = math_round(num)

			if math_abs(num) % 1 == 0.5 and math_floor(math_abs(num) % 2) == 0 then
				result = result - math_sign(result)
			end

			return result
		end

		neg.f64 = neg.f32
		min.f64 = min.f32
		max.f64 = max.f32
		copysign.f64 = copysign.f32
		nearest.f64 = nearest.f32

		module.add = add
		module.sub = sub
		module.mul = mul
		module.div = div
		module.rem = rem
		module.neg = neg
		module.min = min
		module.max = max
		module.copysign = copysign
		module.nearest = nearest
	end

	do
		local popcnt = {}

		function popcnt.i32(num)
			local count = 0

			while num ~= 0 do
				num = bit_and(num, num - 1)
				count = count + 1
			end

			return count
		end

		module.popcnt = popcnt
	end

	do
		local eq = {}
		local ne = {}
		local le = {}
		local lt = {}
		local ge = {}
		local gt = {}

		local num_is_equal = I64.is_equal
		local num_is_less_signed = I64.is_less_signed
		local num_is_less_unsigned = I64.is_less_unsigned
		local num_is_greater_signed = I64.is_greater_signed
		local num_is_greater_unsigned = I64.is_greater_unsigned

		function le.i32(lhs, rhs)
			return to_i32(lhs) <= to_i32(rhs)
		end

		function lt.i32(lhs, rhs)
			return to_i32(lhs) < to_i32(rhs)
		end

		function ge.i32(lhs, rhs)
			return to_i32(lhs) >= to_i32(rhs)
		end

		function gt.i32(lhs, rhs)
			return to_i32(lhs) > to_i32(rhs)
		end

		eq.i64 = num_is_equal

		function ne.i64(lhs, rhs)
			return not num_is_equal(lhs, rhs)
		end

		function le.i64(lhs, rhs)
			return num_is_less_signed(lhs, rhs) or num_is_equal(lhs, rhs)
		end

		function le.u64(lhs, rhs)
			return num_is_less_unsigned(lhs, rhs) or num_is_equal(lhs, rhs)
		end

		lt.i64 = num_is_less_signed
		lt.u64 = num_is_less_unsigned

		function ge.i64(lhs, rhs)
			return num_is_greater_signed(lhs, rhs) or num_is_equal(lhs, rhs)
		end

		function ge.u64(lhs, rhs)
			return num_is_greater_unsigned(lhs, rhs) or num_is_equal(lhs, rhs)
		end

		gt.i64 = num_is_greater_signed
		gt.u64 = num_is_greater_unsigned

		module.eq = eq
		module.ne = ne
		module.le = le
		module.lt = lt
		module.ge = ge
		module.gt = gt
	end

	do
		local band = {}
		local bor = {}
		local bxor = {}
		local bnot = {}

		band.i64 = I64.bit_and
		bor.i64 = I64.bit_or
		bxor.i64 = I64.bit_xor
		bnot.i64 = I64.bit_not

		module.band = band
		module.bor = bor
		module.bxor = bxor
		module.bnot = bnot
	end

	do
		local shl = {}
		local shr = {}
		local rotl = {}
		local rotr = {}

		local bit_arshift = bit32.arshift
		local bit_lrotate = bit32.lrotate
		local bit_rrotate = bit32.rrotate

		function shl.i32(lhs, rhs)
			return bit_lshift(lhs, rhs % 32)
		end

		function shr.u32(lhs, rhs)
			return bit_rshift(lhs, rhs % 32)
		end

		function shr.i32(lhs, rhs)
			return bit_arshift(lhs, rhs % 32)
		end

		function rotl.i32(lhs, rhs)
			return bit_lrotate(lhs, rhs % 32)
		end

		function rotr.i32(lhs, rhs)
			return bit_rrotate(lhs, rhs % 32)
		end

		shl.i64 = I64.shift_left
		shr.i64 = I64.shift_right_signed
		shr.u64 = I64.shift_right_unsigned

		module.shl = shl
		module.shr = shr
		module.rotl = rotl
		module.rotr = rotr
	end

	do
		local wrap = {}
		local trunc = {}
		local extend = {}
		local convert = {}
		local demote = {}
		local promote = {}
		local reinterpret = {}

		local math_ceil = math.ceil
		local math_floor = math.floor

		local string_pack = string.pack
		local string_unpack = string.unpack

		local num_from_u64 = I64.from_u64
		local num_into_u64 = I64.into_u64

		local num_negate = I64.negate
		local num_is_negative = I64.is_negative

		function wrap.i32_i64(num)
			local data_1, _ = num_into_u32(num)

			return data_1
		end

		trunc.i32_f32 = to_u32
		trunc.i32_f64 = to_u32
		trunc.u32_f32 = no_op
		trunc.u32_f64 = no_op

		function trunc.i64_f32(num)
			if num < 0 then
				local temp = num_from_u64(-math_ceil(num))

				return num_negate(temp)
			else
				local temp = math_floor(num)

				return num_from_u64(temp)
			end
		end

		function trunc.i64_f64(num)
			if num < 0 then
				local temp = num_from_u64(-math_ceil(num))

				return num_negate(temp)
			else
				local temp = math_floor(num)

				return num_from_u64(temp)
			end
		end

		function trunc.f32(num)
			if num >= 0 then
				return math_floor(num)
			else
				return math_ceil(num)
			end
		end

		trunc.f64 = trunc.f32
		trunc.u64_f32 = num_from_u64
		trunc.u64_f64 = num_from_u64

		function extend.i64_i32(num)
			if num > MAX_SIGNED then
				local temp = num_from_u32(-num + BIT_SET_32, 0)

				return num_negate(temp)
			else
				return num_from_u32(num, 0)
			end
		end

		function extend.u64_i32(num)
			return num_from_u32(num, 0)
		end

		convert.f32_i32 = no_op
		convert.f32_u32 = no_op

		function convert.f32_i64(num)
			if num_is_negative(num) then
				local temp = num_negate(num)

				return -num_into_u64(temp)
			else
				return num_into_u64(num)
			end
		end

		convert.f32_u64 = num_into_u64
		convert.f64_i32 = to_i32
		convert.f64_u32 = no_op

		function convert.f64_i64(num)
			if num_is_negative(num) then
				local temp = num_negate(num)

				return -num_into_u64(temp)
			else
				return num_into_u64(num)
			end
		end

		convert.f64_u64 = num_into_u64

		demote.f32_f64 = no_op

		promote.f64_f32 = no_op

		function reinterpret.i32_f32(num)
			local packed = string_pack("f", num)

			return string_unpack("I4", packed)
		end

		function reinterpret.i64_f64(num)
			local packed = string_pack("d", num)
			local data_1, data_2 = string_unpack("I4I4", packed)

			return num_from_u32(data_1, data_2)
		end

		function reinterpret.f32_i32(num)
			local packed = string_pack("I4", num)

			return string_unpack("f", packed)
		end

		function reinterpret.f64_i64(num)
			local data_1, data_2 = num_into_u32(num)
			local packed = string_pack("I4I4", data_1, data_2)

			return string_unpack("d", packed)
		end

		module.wrap = wrap
		module.trunc = trunc
		module.extend = extend
		module.convert = convert
		module.demote = demote
		module.promote = promote
		module.reinterpret = reinterpret
	end

	do
		local load = {}
		local store = {}
		local allocator = {}

		local bit_extract = bit32.extract
		local bit_replace = bit32.replace

		local math_floor = math.floor

		local string_byte = string.byte
		local string_unpack = string.unpack

		local reinterpret_f32_i32 = module.reinterpret.f32_i32
		local reinterpret_f64_i64 = module.reinterpret.f64_i64
		local reinterpret_i32_f32 = module.reinterpret.i32_f32
		local reinterpret_i64_f64 = module.reinterpret.i64_f64

		local function load_byte(data, addr)
			local value = data[math_floor(addr / 4)] or 0

			return bit_extract(value, addr % 4 * 8, 8)
		end

		local function store_byte(data, addr, value)
			local adjust = math_floor(addr / 4)

			data[adjust] = bit_replace(data[adjust] or 0, value, addr % 4 * 8, 8)
		end

		function load.i32_i8(memory, addr)
			local temp = load_byte(memory.data, addr)

			if temp >= 0x80 then
				return to_u32(temp - 0x100)
			else
				return temp
			end
		end

		function load.i32_u8(memory, addr)
			return load_byte(memory.data, addr)
		end

		function load.i32_i16(memory, addr)
			local data = memory.data
			local temp

			if addr % 4 == 0 then
				temp = bit_and(data[addr / 4] or 0, 0xFFFF)
			else
				local b1 = load_byte(data, addr)
				local b2 = bit_lshift(load_byte(data, addr + 1), 8)

				temp = bit_or(b1, b2)
			end

			if temp >= 0x8000 then
				return to_u32(temp - 0x10000)
			else
				return temp
			end
		end

		function load.i32_u16(memory, addr)
			local data = memory.data

			if addr % 4 == 0 then
				return bit_and(data[addr / 4] or 0, 0xFFFF)
			else
				local b1 = load_byte(data, addr)
				local b2 = bit_lshift(load_byte(data, addr + 1), 8)

				return bit_or(b1, b2)
			end
		end

		function load.i32(memory, addr)
			local data = memory.data

			if addr % 4 == 0 then
				-- aligned read
				return data[addr / 4] or 0
			else
				-- unaligned read
				local b1 = load_byte(data, addr)
				local b2 = bit_lshift(load_byte(data, addr + 1), 8)
				local b3 = bit_lshift(load_byte(data, addr + 2), 16)
				local b4 = bit_lshift(load_byte(data, addr + 3), 24)

				return bit_or(b1, b2, b3, b4)
			end
		end

		function load.i64_i8(memory, addr)
			local data_1 = load_byte(memory.data, addr)
			local data_2

			if data_1 >= 0x80 then
				data_1 = to_u32(data_1 - 0x100)
				data_2 = MAX_UNSIGNED
			else
				data_2 = 0
			end

			return num_from_u32(data_1, data_2)
		end

		function load.i64_u8(memory, addr)
			local temp = load_byte(memory.data, addr)

			return num_from_u32(temp, 0)
		end

		function load.i64_i16(memory, addr)
			local data = memory.data
			local data_1, data_2

			if addr % 4 == 0 then
				data_1 = bit_and(data[addr / 4] or 0, 0xFFFF)
			else
				local b1 = load_byte(data, addr)
				local b2 = bit_lshift(load_byte(data, addr + 1), 8)

				data_1 = bit_or(b1, b2)
			end

			if data_1 >= 0x8000 then
				data_1 = to_u32(data_1 - 0x10000)
				data_2 = MAX_UNSIGNED
			else
				data_2 = 0
			end

			return num_from_u32(data_1, data_2)
		end

		function load.i64_u16(memory, addr)
			local data = memory.data
			local temp

			if addr % 4 == 0 then
				temp = bit_and(data[addr / 4] or 0, 0xFFFF)
			else
				local b1 = load_byte(data, addr)
				local b2 = bit_lshift(load_byte(data, addr + 1), 8)

				temp = bit_or(b1, b2)
			end

			return num_from_u32(temp, 0)
		end

		function load.i64_i32(memory, addr)
			local data = memory.data
			local data_1, data_2

			if addr % 4 == 0 then
				data_1 = data[addr / 4] or 0
			else
				local b1 = load_byte(data, addr)
				local b2 = bit_lshift(load_byte(data, addr + 1), 8)
				local b3 = bit_lshift(load_byte(data, addr + 2), 16)
				local b4 = bit_lshift(load_byte(data, addr + 3), 24)

				data_1 = bit_or(b1, b2, b3, b4)
			end

			if data_1 >= 0x80000000 then
				data_1 = to_u32(data_1 - 0x100000000)
				data_2 = MAX_UNSIGNED
			else
				data_2 = 0
			end

			return num_from_u32(data_1, data_2)
		end

		function load.i64_u32(memory, addr)
			local data = memory.data
			local temp

			if addr % 4 == 0 then
				temp = data[addr / 4] or 0
			else
				local b1 = load_byte(data, addr)
				local b2 = bit_lshift(load_byte(data, addr + 1), 8)
				local b3 = bit_lshift(load_byte(data, addr + 2), 16)
				local b4 = bit_lshift(load_byte(data, addr + 3), 24)

				temp = bit_or(b1, b2, b3, b4)
			end

			return num_from_u32(temp, 0)
		end

		local load_i32 = load.i32

		function load.i64(memory, addr)
			local data_1 = load_i32(memory, addr)
			local data_2 = load_i32(memory, addr + 4)

			return num_from_u32(data_1, data_2)
		end

		local load_i64 = load.i64

		function load.f32(memory, addr)
			local raw = load_i32(memory, addr)

			return reinterpret_f32_i32(raw)
		end

		function load.f64(memory, addr)
			local raw = load_i64(memory, addr)

			return reinterpret_f64_i64(raw)
		end

		function store.i32_n8(memory, addr, value)
			store_byte(memory.data, addr, value)
		end

		local store_i8 = store.i32_n8

		function store.i32_n16(memory, addr, value)
			store_byte(memory.data, addr, value)
			store_byte(memory.data, addr + 1, bit_rshift(value, 8))
		end

		function store.i32(memory, addr, value)
			local data = memory.data

			if addr % 4 == 0 then
				-- aligned write
				data[addr / 4] = value
			else
				-- unaligned write
				store_byte(data, addr, value)
				store_byte(data, addr + 1, bit_rshift(value, 8))
				store_byte(data, addr + 2, bit_rshift(value, 16))
				store_byte(data, addr + 3, bit_rshift(value, 24))
			end
		end

		local store_i32 = store.i32
		local store_i32_n8 = store.i32_n8
		local store_i32_n16 = store.i32_n16

		function store.i64_n8(memory, addr, value)
			local data_1, _ = num_into_u32(value)

			store_i32_n8(memory, addr, data_1)
		end

		function store.i64_n16(memory, addr, value)
			local data_1, _ = num_into_u32(value)

			store_i32_n16(memory, addr, data_1)
		end

		function store.i64_n32(memory, addr, value)
			local data_1, _ = num_into_u32(value)

			store_i32(memory, addr, data_1)
		end

		function store.i64(memory, addr, value)
			local data_1, data_2 = num_into_u32(value)

			store_i32(memory, addr, data_1)
			store_i32(memory, addr + 4, data_2)
		end

		local store_i64 = store.i64

		function store.f32(memory, addr, value)
			store_i32(memory, addr, reinterpret_i32_f32(value))
		end

		function store.f64(memory, addr, value)
			store_i64(memory, addr, reinterpret_i64_f64(value))
		end

		function store.string(memory, offset, data, len)
			len = len or #data

			local rem = len % 4

			for i = 1, len - rem, 4 do
				local v = string_unpack("<I4", data, i)

				store_i32(memory, offset + i - 1, v)
			end

			for i = len - rem + 1, len do
				local v = string_byte(data, i)

				store_i8(memory, offset + i - 1, v)
			end
		end

		function allocator.new(min, max)
			return { min = min, max = max, data = {} }
		end

		function allocator.grow(memory, num)
			local old = memory.min
			local new = old + num

			if new > memory.max then
				return to_u32(-1)
			else
				memory.min = new

				return old
			end
		end

		module.load = load
		module.store = store
		module.allocator = allocator
	end

	return module
end)()

return rt
