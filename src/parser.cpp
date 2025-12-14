#include "../include/parser.hpp"

#include <optional>
#include <ranges>

#include <cctype>

template<class... Ts> struct Overloaded : Ts... { using Ts::operator()...; };

float Lexer::get_binding_power(char op) {
    switch (op) {
        case '+':
        case '-':
            return 1.0;
        case '*':
        case '/':
            return 2.0;
        default:
            exit(69);
    }
}

Token Lexer::next() {
    if (tokens.size() == 0) {
        return Eof {};
    }

    Token token = tokens.back();
    tokens.pop_back();
    return token;
}

Token Lexer::peek() {
    if (tokens.size() == 0) {
        return Eof {};
    }

    return tokens.back();
}

Expression Lexer::parse_expression(float min_bp) {
    Expression lhs = std::visit(Overloaded{
        [](Atom atom) { return Expression(atom); },
        [](auto&& other) -> Expression { exit(2); }
    }, next());

    while (true) {
        std::optional<Op> op_opt = std::visit(Overloaded{
            [](Eof eof) { return std::optional<Op>{}; },
            [](Op op) { return std::make_optional(op); },
            [](auto&& other) -> std::optional<Op> { exit(3); }
        }, peek());

        Op op;
        if (op_opt) {
            op = *op_opt;
        } else {
            break;
        }

        float bp = get_binding_power(op.c);
        if (bp < min_bp) {
            break;
        }
        
        next();

        Expression rhs = parse_expression(bp + 0.1f);
        lhs = Expression(Operation {op, std::vector {lhs, std::move(rhs)}});
    }

    return lhs;
}

void Lexer::tokenize(std::string_view input) {
    tokens = input
        | std::views::filter([](char c){ return !std::isspace(c); })
        | std::views::transform([](char c){
            return (std::isalpha(c) || std::isdigit(c)) ? Token(Atom { c }) : Token(Op { c }); 
        })
        | std::views::reverse
        | std::ranges::to<std::vector>();
}
