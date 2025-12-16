const std = @import("std");
const types = @import("types.zig");
const utils = @import("utils.zig");
const Value = types.Value;

const ParseError = error{
    UnknownOperator,
    InvalidToken,
};

const Token = union(enum) {
    atom: u8,
    op: u8,
    eof,
};

pub const Expression = union(enum) {
    atom: u8,
    operation: Operation,

    fn to_string(self: *const Expression, allocator: std.mem.Allocator) ![]u8 {
        switch (self.*) {
            .atom => |val| {
                // Convert single atom (u8) to string
                return std.fmt.allocPrint(allocator, "{c}", .{val});
            },
            .operation => |op| {
                // Start with operator
                var parts = try std.ArrayList([]u8).initCapacity(allocator, 1);
                try parts.append(allocator, try std.fmt.allocPrint(allocator, "{c}", .{op.op}));

                // Append each sub-expression
                for (op.expressions.items) |expr| {
                    const sub = try expr.to_string(allocator);
                    try parts.append(allocator, sub);
                }

                // Concatenate all parts into one string
                return try std.mem.concat(allocator, u8, parts.items);
            },
            //else => unreachable,
        }
    }

    pub fn evaluate(self: *const Expression) (utils.ConversionError || Value.OperationError)!Value {
        switch (self.*) {
            .atom => |a| return utils.stringToValue(&[_]u8{a}),
            .operation => |op| return op.evaluate(),
        }
    }
};

const Operation = struct {
    op: u8,
    expressions: std.ArrayList(Expression),

    fn evaluate(self: *const Operation) (utils.ConversionError || Value.OperationError)!Value {
        switch (self.op) {
            '+' => return Value.add(
                try self.expressions.items[0].evaluate(),
                try self.expressions.items[1].evaluate(),
            ),
            '*' => return Value.mul(
                try self.expressions.items[0].evaluate(),
                try self.expressions.items[1].evaluate(),
            ),
            else => unreachable,
        }
    }
};

pub const Lexer = struct {
    tokens: std.ArrayList(Token),

    fn getBindingPower(op: u8) !f32 {
        switch (op) {
            '+', '-' => return 1.0,
            '*', '/' => return 2.0,
            else => return ParseError.InvalidToken,
        }
    }

    fn next(self: *Lexer) Token {
        return self.tokens.pop() orelse Token.eof;
    }

    fn peek(self: *Lexer) Token {
        return self.tokens.getLastOrNull() orelse Token.eof;
    }

    pub fn parseExpression(self: *Lexer, gpa: std.mem.Allocator, min_bp: f32) !Expression {

        //std.log.warn("\n{c}z\n", .{self.tokens.getLast().atom});
        var lhs: Expression = try switch (self.next()) {
            .atom => |a| Expression{ .atom = a },
            .op => |op| if (op == '(') try self.parseExpression(gpa, 0.0) else ParseError.InvalidToken,
            else => ParseError.InvalidToken,
        };
        //std.log.warn("\n{c}z\n", .{self.tokens.getLast().op});
        //_ = self.next();

        //switch (self.peek()) {
        //    .atom => |c| std.log.warn("\n{c}z\n", .{c}),
        //    .op => |c| std.log.warn("\n{c}a\n", .{c}),
        //    else => unreachable,
        //}

        while (true) {
            const op = switch (self.peek()) {
                .eof => break,
                .op => |op| if (op == ')') break else op,
                .atom => |a| {
                    std.log.warn("\n\na{c}a\n\n", .{a});
                    return ParseError.InvalidToken;
                },
                //else => ParseError.InvalidToken,
            };

            const bp = try getBindingPower(op);

            if (bp < min_bp) {
                break;
            }

            _ = self.next();

            const rhs = try self.parseExpression(gpa, bp + 0.1);

            var expressions = try std.ArrayList(Expression).initCapacity(gpa, 2);
            try expressions.append(gpa, lhs);
            try expressions.append(gpa, rhs);

            lhs = Expression{ .operation = .{
                .op = op,
                .expressions = expressions,
            } };
        }

        return lhs;
    }

    pub fn new(gpa: std.mem.Allocator) !Lexer {
        return Lexer{ .tokens = try std.ArrayList(Token).initCapacity(gpa, 0) };
    }

    pub fn tokenize(self: *Lexer, gpa: std.mem.Allocator, input: []const u8) !void {
        for (0..input.len) |i| {
            const c = input[input.len - i - 1];

            if (std.ascii.isWhitespace(c)) continue;

            if (std.ascii.isAlphanumeric(c)) {
                try self.tokens.append(gpa, Token{ .atom = c });
            } else {
                try self.tokens.append(gpa, Token{ .op = c });
            }
        }
    }
};

test "Tokenize expression" {
    //const expect = std.testing.expect;
    const test_allocator = std.testing.allocator;

    const test_expression = "2 + 6 + 2";

    var lexer = try Lexer.new(test_allocator);
    try lexer.tokenize(test_allocator, test_expression);
    //std.debug.print("{s}", .{});
    const expr = try lexer.parseExpression(test_allocator, -std.math.inf(f32));
    std.log.warn("{s}", .{try expr.to_string(test_allocator)});
    switch (expr) {
        .atom => std.log.warn("atom", .{}),
        .operation => std.log.warn("operation", .{}),
    }
}

//void Lexer::tokenize(std::string_view input) {
//    tokens = input
//        | std::views::filter([](char c){ return !std::isspace(c); })
//        | std::views::transform([](char c){
//            return (std::isalpha(c) || std::isdigit(c)) ? Token(Atom { c }) : Token(Op { c });
//        })
//        | std::views::reverse
//        | std::ranges::to<std::vector>();
//}
