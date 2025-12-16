const std = @import("std");

pub const Type = enum {
    int,
    float,
    bool,
    function,
    type,
    class,
};

pub const Value = union(Type) {
    pub const TypeError = error{
        MismatchedTypes,
    };

    pub const OperationError = TypeError || error{
        NoFunctionAvailable,
    };

    int: i64,
    float: f64,
    bool: bool,
    function: *FunctionDefinition,
    type: Type,
    class: *ClassInstance,

    pub fn add(self: Value, other: Value) OperationError!Value {
        switch (self) {
            .int => if (other != .int) return OperationError.MismatchedTypes else return Value{ .int = self.int + other.int },
            .float => if (other != .float) return OperationError.MismatchedTypes else return Value{ .float = self.float + other.float },
            else => return OperationError.NoFunctionAvailable,
        }
    }

    pub fn mul(self: Value, other: Value) OperationError!Value {
        switch (self) {
            .int => if (other != .int) return OperationError.MismatchedTypes else return Value{ .int = self.int * other.int },
            .float => if (other != .float) return OperationError.MismatchedTypes else return Value{ .float = self.float * other.float },
            else => return OperationError.NoFunctionAvailable,
        }
    }
};

pub const FunctionDefinition = struct {
    name: []u8,
    parameters: std.ArrayList(Type),
    return_type: Type,
};

pub const ClassDefinition = struct {
    name: []u8,
    functions: std.ArrayListUnmanaged(FunctionDefinition),
};

pub const EnumDefinition = struct {
    name: []u8,
};

pub const ClassInstance = struct {
    defintion: *ClassDefinition,
    instance_variables: std.StringHashMapUnmanaged(Value),
};

pub const Enum = struct {};
