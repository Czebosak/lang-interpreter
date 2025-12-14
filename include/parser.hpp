#pragma once

#include <string_view>
#include <vector>
#include <variant>

struct Atom { char c; };
struct Op { char c; };
struct Eof {};

using Token = std::variant<Atom, Op, Eof>;

struct Operation;

using Expression = std::variant<Atom, Operation>;

struct Operation {
    Op op;
    std::vector<Expression> expressions;
};

class Lexer {
private:
    std::vector<Token> tokens;

    float get_binding_power(char op);

    Token next();
    Token peek();

    Expression parse_expression(float min_bp);
public:
    void tokenize(std::string_view input);
};
