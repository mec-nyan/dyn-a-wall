//! Zig seems cool.  Let's give it a go!
//!
//!

const std = @import("std");

pub fn main() !void {
    var args = std.process.args();

    _ = args.next();

    var counter = args;
    var count: usize = 0;
    while (counter.next() != null) : (count += 1) {}

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Number of arguments is {d}.\n", .{count});
    while (args.next()) |arg| {
        try stdout.print("\t- {s}\n", .{arg});
    }
}
