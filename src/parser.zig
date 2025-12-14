const std = @import("std");

const Token = union(enum) {
    atom: u8,
    op: u8,
    eof,
};

const Expression = union(enum) {
    atom: u8,
    operation: Operation,
};

const Operation = struct {
    op: u8,
    expressions: std.ArrayList(Expression),
};

const Lexer = struct {
    tokens: std.ArrayList(Token),

    fn tokenize(self: *Lexer, allocator: std.mem.Allocator, input: []const u8) !void {
        self.tokens = try std.ArrayList(u8).initCapacity(allocator, 0);

        for (input) |c| {
            if (std.ascii.isWhitespace(c)) continue;

            if (std.ascii.isAlphanumeric(c)) {
                try self.tokens.append(Token{ .atom = c });
            } else {
                try self.tokens.append(Token{ .op = c });
            }
        }

        std.mem.reverse(Token, self.tokens.items);

        for (tokens.items) |tok| {
            switch (tok) {
                .Atom => |a| std.debug.print("Atom({c})\n", .{a.c}),
                .Op => |o| std.debug.print("Op({c})\n", .{o.c}),
            }
        }
    }
};

//void Lexer::tokenize(std::string_view input) {
//    tokens = input
//        | std::views::filter([](char c){ return !std::isspace(c); })
//        | std::views::transform([](char c){
//            return (std::isalpha(c) || std::isdigit(c)) ? Token(Atom { c }) : Token(Op { c });
//        })
//        | std::views::reverse
//        | std::ranges::to<std::vector>();
//}
