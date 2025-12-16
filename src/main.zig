const std = @import("std");
const parser = @import("parser.zig");
const types = @import("types.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const gpa = arena.allocator();

    //const input = try std.fs.cwd().readFileAlloc(gpa, "test", 128_000_000);

    const input = "2 + 6* 3";

    var lexer = try parser.Lexer.new(gpa);
    try lexer.tokenize(gpa, input);

    const expr = try lexer.parseExpression(gpa, -std.math.inf(f32));
    const value = try expr.evaluate();

    std.debug.print("{}", .{value.int});
}
