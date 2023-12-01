const std = @import("std");

fn part_one(input: []const u8) ?u32 {
    // your code here
    return null;
} 

fn part_two(input: []const u8) ?u32 {
    // your code here
    return null;
}

const part = enum {
    part_one = 1,
    part_two = 2,
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = @embedFile("data/input.txt");

    var args = std.process.args();
    _ = args.skip(); // skip the program name
    const part = args.next().?[0];

    switch (part) {
        '1' => {
            const part_one_result = part_one(input);
            try stdout.print("{}\n", .{part_one_result.?});
        },
        '2' => {
            const part_two_result = part_two(input);
            try stdout.print("{}\n", .{part_two_result.?});
        },
        else => {
            std.debug.print("Please specify a part to run (1 or 2)\n", .{});
            std.process.exit(1);
        },
    }
}

test "part 1" {
    const input = @embedFile("data/example.txt");
    try std.testing.expectEqual(part_one(input), null); // change null to the expected result
}
test "part 2" {
    const input = @embedFile("data/example.txt");
    try std.testing.expectEqual(part_two(input), null); // change null to the expected result
}



