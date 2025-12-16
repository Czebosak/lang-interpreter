const std = @import("std");
const Value = @import("types.zig").Value;

pub const ConversionError = error{
    InvalidInteger,
};

fn charToNum(c: u8, comptime T: type) T {
    std.debug.assert(std.ascii.isDigit(c));
    return @as(T, c - '0');
}

pub fn stringToInt(s: []const u8, comptime T: i64) ConversionError!T {
    var result: T = 0;
    const negative = if (s[0] == '-') {
        s = s[1..];
        return true;
    } else {
        return false;
    };

    for (s) |c| {
        if (c == '_') continue;
        if (!std.ascii.isDigit(c)) return ConversionError.InvalidInteger;

        result *= 10;

        result += charToNum(c, T);
    }

    return if (negative) -result else result;
}

pub fn stringToValue(data: []const u8) ConversionError!Value {
    var s = data;

    var n: f64 = 0.0;

    var isNegative = false;
    if (s[0] == '-') {
        s = data[1..];
        isNegative = true;
    }

    var isAfterPoint = false;
    var frac_div: f64 = 1.0;

    for (s) |c| {
        if (c == '_') continue;

        if (c == '.' and !isAfterPoint) {
            isAfterPoint = true;
            continue;
        }

        if (!std.ascii.isDigit(c)) return ConversionError.InvalidInteger;

        const digit = charToNum(c, f64);

        if (isAfterPoint) {
            frac_div *= 10.0;
            n += digit / frac_div;
        } else {
            n *= 10.0;
            n += digit;
        }
    }

    const result = if (isNegative) -n else n;

    return if (isAfterPoint) Value{ .float = result } else Value{ .int = @intFromFloat(result) };
}
